/*pub trait WorkerBuilder: Send + Clone {
  type Worker;
  type Context;

  fn into_worker(self, context: Self::Context) -> Self::Worker;
}*/

#[derive(Clone, Copy)]
pub struct WorkerData {
  worker_rank:  usize,
  num_workers:  usize,
}

impl WorkerData {
  pub fn new(worker_rank: usize, num_workers: usize) -> WorkerData {
    WorkerData{
      worker_rank:  worker_rank,
      num_workers:  num_workers,
    }
  }

  pub fn tid(&self) -> usize {
    self.worker_rank
  }

  pub fn worker_rank(&self) -> usize {
    self.worker_rank
  }

  pub fn num_workers(&self) -> usize {
    self.num_workers
  }
}
