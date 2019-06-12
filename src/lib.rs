use std::hash::Hash;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU16};
use std::thread;
use std::iter;
use std::collections::hash_map::DefaultHasher;



struct KVPair<Key,Value>{key:Key,value:Value}

struct Slot<Key,Value>{
    kvp:Mutex<Option<KVPair<Key,Value>>>, next: Option<AtomicU16>
}

struct Yacht<Key:Hash+Eq,Value>{ 
    slots:Box<[Slot<Key,Value>]>
}

impl<K,V> Default for Slot<K,V>{
    fn default()->Slot<K,V>{
        Slot{kvp:Mutex::new(None),next:None}
    }
}

impl<K:Eq+Hash,V> Yacht<K,V> {
    const STARTING_SLOTS:usize = 32;
}

impl<K:Hash+Eq+Clone,V:Clone> Yacht<K,V>{
    fn new()->Yacht<K,V>{
        
    }
    
    fn put(&self,key:K,value:V){
        let hasher = DefaultHasher::new();
        hasher.write(key);
        let key_hash = hasher.finish();
        let start_slot_idx = key_hash % self.slots.count;
        unimplemented!();
    }
    
    
}


fn main(){
  let mut map = Yacht::new();
  
  let mut handles = vec![];
  
  for n in 0..10 {
    let handle = thread::spawn(move || {
      map.put(n, n+1)
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }
  
}
