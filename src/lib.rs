pub struct WorkerData {
  num_workers:  usize,
  tid:  usize,
}

impl WorkerData {
  pub fn new(num_workers: usize, tid: usize) -> WorkerData {
    WorkerData{
      num_workers:  num_workers,
      tid:  tid,
    }
  }

  pub fn num_workers(&self) -> usize {
    self.num_workers
  }

  pub fn tid(&self) -> usize {
    self.tid
  }
}
