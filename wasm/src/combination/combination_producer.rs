pub trait CombinationProducer<T> {
    // Splits the CombinationProducer into two CombinationProducers, each of which iterate over half
    // (or as close to half as possible) of the combinations of the parent. Returns Err if the
    // CombinationProducer cannot be split.
    fn bisect(self) -> Result<(Self, Self), Self> where Self: Sized;

    // Runs the given function for all combinations of the combination_producer.
    fn forall<F: FnMut(&T)>(&self, f: F);
}
