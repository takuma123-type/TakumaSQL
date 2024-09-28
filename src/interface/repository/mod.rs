// src/interface/repository/mod.rs

pub struct InMemoryTableRepository;

impl InMemoryTableRepository {
    pub fn new() -> Self {
        InMemoryTableRepository
    }
}

pub struct InMemoryRecordRepository;

impl InMemoryRecordRepository {
    pub fn new() -> Self {
        InMemoryRecordRepository
    }
}
