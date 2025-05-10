use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SummandStrategy {
    run_strategy: RunStrategies,
}

impl SummandStrategy {
    pub fn new(run_strategy: RunStrategies) -> SummandStrategy {
        Self {
            run_strategy: run_strategy,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RunStrategies {
    Ignore(RunIgnoreStrategy),
    Retry(RunRetryStrategy),
    Break(RunBreakStrategy),
}

#[derive(Debug)]
pub struct RunStrategyError {
    message: String,
}

impl std::error::Error for RunStrategyError {}

impl fmt::Display for RunStrategyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Run Strategy Error({})", self.message)
    }
}

pub trait RunStrategy {
    fn handle(&mut self) -> Result<(), RunStrategyError>;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RunRetryStrategy {
    max_retry: i32,
    retry_count: i32,
}
impl RunRetryStrategy {
    pub fn new(max_retry: Option<i32>) -> RunRetryStrategy {
        Self {
            max_retry: max_retry.or(Some(2)).unwrap(),
            retry_count: 0,
        }
    }
}
impl RunStrategy for RunRetryStrategy {
    fn handle(&mut self) -> Result<(), RunStrategyError> {
        if self.retry_count > self.max_retry {
            return Err(RunStrategyError {
                message: "Max retry exceeded".to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RunBreakStrategy {}
impl RunBreakStrategy {
    pub fn new() -> RunBreakStrategy {
        Self {}
    }
}
impl RunStrategy for RunBreakStrategy {
    fn handle(&mut self) -> Result<(), RunStrategyError> {
        return Err(RunStrategyError {
            message: "Break on error".to_string(),
        });
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RunIgnoreStrategy {}
impl RunIgnoreStrategy {
    pub fn new() -> RunIgnoreStrategy {
        Self {}
    }
}
impl RunStrategy for RunIgnoreStrategy {
    fn handle(&mut self) -> Result<(), RunStrategyError> {
        Ok(())
    }
}
