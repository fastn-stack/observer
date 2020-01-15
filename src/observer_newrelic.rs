use failure::_core::cell::RefCell;

pub struct ObserverNewRelic {
    segment_stack: RefCell<Vec<ackorelic::acko_segment::Segment>>,
}

impl ObserverNewRelic {
    //    fn new() -> Self {
    //        ObserverNewRelic {
    //            segment_stack: RefCell::new(vec![]),
    //        }
    //    }
}
/// Implementation of Backend trait for NewRelic

impl crate::Backend for ObserverNewRelic {
    /// This will start NewRelic app
    fn app_started(&self) {}
    /// This will end NewRelic app
    fn app_ended(&self) {}
    /// This method will be called when context has been created.
    fn context_created(&self, id: &str) {
        // Need to create web transaction of NewRelic
        // ackorelic::newrelic_fn::nr_start_web_transaction(id);
        println!("ObserverNewRelic: Context Started: {}", id);
    }
    /// This method will be called when context ended.
    fn context_ended(&self) {
        // Need to end web transaction
        // ackorelic::newrelic_fn::nr_end_transaction()
        println!("ObserverNewRelic: Context Ended")
    }
    /// This method will be when span created.
    fn span_created(&self, id: &str) {
        // Need to start a newrelic segment and store it stack
        //        self.segment_stack
        //            .borrow_mut()
        //            .push(ackorelic::newrelic_fn::nr_start_custom_segment(id))
        println!("ObserverNewRelic: Span Created: {}", id)
    }
    /// This method will be when span needs to logged.
    fn span_data(&self, _key: &str, _value: &str) {}
    /// This method will be when span ended.
    fn span_ended(&self) {
        // Needs to end a segment which was stored earlier in stack
        //        if let Some(segment) = self.segment_stack.borrow_mut().pop() {
        //            ackorelic::newrelic_fn::nr_end_custom_segment(segment);
        //        }
        println!("ObserverNewRelic: Span Ended")
    }
}

fn _test() {
    // let _t: Box<dyn crate::Backend> = Box::new(ObserverNewRelic::new());
}
