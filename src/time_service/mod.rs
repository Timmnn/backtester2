use chrono::NaiveTime;

pub trait TimeService {
    fn get_current_time(&self) -> chrono::NaiveDateTime;
}

// BacktestTimeService

pub struct BacktestTimeService {
    time: chrono::NaiveDateTime,
}

impl BacktestTimeService {
    pub fn new() -> Self {
        Self {
            time: chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
                .unwrap()
                .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
        }
    }

    pub fn set_time(&mut self, time: chrono::NaiveDateTime) {
        self.time = time
    }
}

impl TimeService for BacktestTimeService {
    fn get_current_time(&self) -> chrono::NaiveDateTime {
        self.time
    }
}

// LiveTimeService

pub struct LiveTimeService {}

impl TimeService for LiveTimeService {
    fn get_current_time(&self) -> chrono::NaiveDateTime {
        chrono::offset::Local::now().naive_local()
    }
}
