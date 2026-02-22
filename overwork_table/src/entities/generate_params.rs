pub enum WorkShift {
    IsNight,
    IsDay,
}

pub struct GenerateParams {
    salary: u32,
    shift: Option<WorkShift>,
}

impl GenerateParams {
    pub fn new(salary: u32) -> Self {
        Self {
            salary: salary,
            shift: None,
        }
    }

    pub fn with_shift(mut self, shift: WorkShift) -> Self {
        self.shift = Some(shift);
        self
    }

    pub fn salary(&self) -> u32 {
        self.salary
    }

    pub fn shift(&self) -> &Option<WorkShift> {
        &self.shift
    }
}
