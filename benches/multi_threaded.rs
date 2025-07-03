use criterion::{Criterion, criterion_group, criterion_main};
use kvs::thread_pool::{RayonThreadPool, SharedQueueThreadPool, ThreadPool};
use kvs::{KvStore, KvsClient, KvsEngine, KvsServer, SledKvsEngine};
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Barrier},
    thread,
    time::Duration,
};
use tempfile::TempDir;

const NUM_CLIENTS: usize = 100;

fn write_benchmark<E, P>(name: &str, c: &mut Criterion, engine: Engine, pool: Pool)
where
    E: KvsEngine + Clone + Send + 'static,
    P: ThreadPool + Send + 'static,
{
    let mut group = c.benchmark_group(name);
    let thread_counts = thread_counts();

    for &num_threads in &thread_counts {
        group.bench_function(format!("{name}_{}threads", num_threads), |b| {
            b.iter_custom(|iters| {
                let mut total_duration = Duration::ZERO;
                for _ in 0..iters {
                    let addr: SocketAddr = "127.0.0.1:4000".parse().unwrap();
                    dispatch_server(&engine, &pool, addr);

                    let barrier = Arc::new(Barrier::new(NUM_CLIENTS + 1));
                    let start = std::time::Instant::now();
                    let mut handles = Vec::new();
                    for i in 0..NUM_CLIENTS {
                        let b = barrier.clone();
                        handles.push(thread::spawn(move || {
                            let mut client = KvsClient::connect(addr).unwrap();
                            client.set(format!("key{i}"), "value".to_string()).unwrap();
                            b.wait();
                        }));
                    }
                    barrier.wait(); // allow timing to include all client work
                    for h in handles {
                        h.join().unwrap();
                    }
                    total_duration += start.elapsed();
                }
                total_duration
            });
        });
    }

    group.finish();
}

fn read_benchmark<E, P>(name: &str, c: &mut Criterion, engine: Engine, pool: Pool)
where
    E: KvsEngine,
    P: ThreadPool,
{
    let mut group = c.benchmark_group(name);
    let thread_counts = thread_counts();

    for &num_threads in &thread_counts {
        group.bench_function(format!("{name}_{}threads", num_threads), |b| {
            b.iter_custom(|iters| {
                let mut total_duration = Duration::ZERO;
                for _ in 0..iters {
                    let addr: SocketAddr = "127.0.0.1:4001".parse().unwrap();
                    dispatch_server(&engine, &pool, addr);

                    // preload keys
                    {
                        let mut preload_client = KvsClient::connect(addr).unwrap();
                        for i in 0..NUM_CLIENTS {
                            preload_client
                                .set(format!("key{i}"), "value".to_string())
                                .unwrap();
                        }
                    }

                    let barrier = Arc::new(Barrier::new(NUM_CLIENTS + 1));
                    let start = std::time::Instant::now();
                    let mut handles = Vec::new();
                    for i in 0..NUM_CLIENTS {
                        let b = barrier.clone();
                        handles.push(thread::spawn(move || {
                            let mut client = KvsClient::connect(addr).unwrap();
                            let value = client.get(format!("key{i}")).unwrap().unwrap();
                            assert_eq!(value, "value");
                            b.wait();
                        }));
                    }
                    barrier.wait();
                    for h in handles {
                        h.join().unwrap();
                    }
                    total_duration += start.elapsed();
                }
                total_duration
            });
        });
    }

    group.finish();
}

enum Engine {
    Kvs,
    Sled,
}

enum Pool {
    Rayon,
    SharedQueue,
}

fn thread_counts() -> Vec<usize> {
    let num_cpus = num_cpus::get();
    (1..=num_cpus * 2)
        .filter(|n| n % 2 == 0 || *n == 1)
        .collect()
}

fn run_benchmark_server<E, P>(addr: SocketAddr)
where
    E: KvsEngine + Send + 'static,
    P: ThreadPool + Send + 'static,
{
    let temp_dir = TempDir::new().unwrap();
    let path_buf: PathBuf = temp_dir.path().to_path_buf();
    let engine = E::open(&path_buf).unwrap();
    let cpus = num_cpus::get() as u32;
    let pool = P::new(cpus).unwrap();

    thread::spawn(move || {
        if let Err(e) = KvsServer::new(engine, pool).run(addr) {
            eprintln!("Server error: {}", e);
        }
    });

    thread::sleep(Duration::from_millis(200));
}

fn dispatch_server(engine: &Engine, pool: &Pool, addr: SocketAddr) {
    match (engine, pool) {
        (Engine::Kvs, Pool::SharedQueue) => {
            run_benchmark_server::<KvStore, SharedQueueThreadPool>(addr)
        }
        (Engine::Kvs, Pool::Rayon) => run_benchmark_server::<KvStore, RayonThreadPool>(addr),
        (Engine::Sled, Pool::SharedQueue) => {
            run_benchmark_server::<SledKvsEngine, SharedQueueThreadPool>(addr)
        }
        (Engine::Sled, Pool::Rayon) => run_benchmark_server::<SledKvsEngine, RayonThreadPool>(addr),
    }
}

fn criterion_benchmarks(c: &mut Criterion) {
    write_benchmark::<KvStore, SharedQueueThreadPool>(
        "write_queued_kvstore",
        c,
        Engine::Kvs,
        Pool::SharedQueue,
    );
    read_benchmark::<KvStore, SharedQueueThreadPool>(
        "read_queued_kvstore",
        c,
        Engine::Kvs,
        Pool::SharedQueue,
    );

    write_benchmark::<KvStore, RayonThreadPool>("write_rayon_kvstore", c, Engine::Kvs, Pool::Rayon);
    read_benchmark::<KvStore, RayonThreadPool>("read_rayon_kvstore", c, Engine::Kvs, Pool::Rayon);

    write_benchmark::<SledKvsEngine, RayonThreadPool>(
        "write_rayon_sled",
        c,
        Engine::Sled,
        Pool::Rayon,
    );
    read_benchmark::<SledKvsEngine, RayonThreadPool>(
        "read_rayon_sled",
        c,
        Engine::Sled,
        Pool::Rayon,
    );

    write_benchmark::<SledKvsEngine, SharedQueueThreadPool>(
        "write_queued_sled",
        c,
        Engine::Sled,
        Pool::SharedQueue,
    );
    read_benchmark::<SledKvsEngine, SharedQueueThreadPool>(
        "read_queued_sled",
        c,
        Engine::Sled,
        Pool::SharedQueue,
    );
}

criterion_group!(benches, criterion_benchmarks);
criterion_main!(benches);
