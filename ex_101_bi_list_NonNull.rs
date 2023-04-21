pub mod bi_list {
    use std::pin::Pin;
    use std::ptr::NonNull;
    use std::marker::PhantomPinned;

    #[derive(Debug)]
    pub struct BiListElement<T> {
        prev: NonNull<BiListElement<T>>,
        next: NonNull<BiListElement<T>>,
//        ctx: NonNull<BiListImpl<T>>,
        val: T,
        _pin: PhantomPinned,
    }

    impl<T> BiListElement<T> {
        fn new(val: T) -> Pin<Box<Self>> {
            let mut s = Box::pin(BiListElement {
                prev: NonNull::dangling(),
                next: NonNull::dangling(),
                val,
//                ctx: NonNull::dangling(),
                _pin: PhantomPinned,
            });
            unsafe {
                let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut s);
                let r = Pin::get_unchecked_mut(mut_ref);
                let nn = NonNull::from(r);

                let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut s);
                let r = Pin::get_unchecked_mut(mut_ref);
                r.prev = nn;

//                let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut s);
//                let r = Pin::get_unchecked_mut(mut_ref);
//                let nn = NonNull::from(r);

                let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut s);
                let r = Pin::get_unchecked_mut(mut_ref);
                r.next = nn;
            }
            s
        }
        pub fn prev(&self) -> &Self {
            unsafe {
                self.prev.as_ref()
            }
        }
        pub fn next(&self) -> &Self {
            unsafe {
                self.next.as_ref()
            }
        }

        pub fn set_val(self: &mut Pin<Box<Self>>, val: T) {
            unsafe {
                let mut_ref: Pin<&mut Self> = Pin::as_mut(self);
                let r = Pin::get_unchecked_mut(mut_ref);
                r.val = val;
            }
        }

        pub fn get_val(&self) -> &T {
            &self.val
        }

        #[allow(dead_code)]
        fn set_next(&mut self, next: NonNull<Self>) {
            self.next = next;
        }
        #[allow(dead_code)]
        fn set_prev(&mut self, prev: NonNull<Self>) {
            self.prev = prev;
        }
    }

    #[derive(Debug)]
    pub struct BiList<T> {
        sack: Vec<Pin<Box<BiListElement<T>>>>,
        begin: Option<NonNull<BiListElement<T>>>,
        end: Option<NonNull<BiListElement<T>>>,
    }

    impl<T> BiList<T> {
        pub fn new() -> Self {
            Self {
                sack: vec![],
                begin: None,
                end: None,
            }
        }

        pub fn push(&mut self, val: T) {
            let mut el = BiListElement::new(val);
            match &mut self.end {
                None => {
                    assert!(self.begin.is_none());
                    assert!(self.sack.is_empty());
                    unsafe {
                        let mut_ref: Pin<&mut _> = Pin::as_mut(&mut el);
                        let r = Pin::get_unchecked_mut(mut_ref);
                        let nn = NonNull::from(r);
                        self.end = Some(nn);
                        self.begin = Some(nn);
                    }
                },
                Some(ex_end) => {
                    unsafe {
                        let mut_ref: Pin<&mut _> = Pin::as_mut(&mut el);
                        let r = Pin::get_unchecked_mut(mut_ref);
                        r.prev = *ex_end;
                        r.next = ex_end.as_ref().next;
                        let nn = NonNull::from(r);
                        if ex_end.as_ref().next != *ex_end {
                            ex_end.as_mut().next.as_mut().prev = nn;
                        }
                        ex_end.as_mut().next = nn;
                        self.end = Some(nn)
                    }
                }
            }
            self.sack.push(el);
        }

        pub fn get_begin(&self) -> Option<&BiListElement<T>> {
            self.begin.map(|nn| unsafe {nn.as_ref()})
        }
        pub fn get_end(&self) -> Option<&BiListElement<T>> {
            self.end.map(|nn| unsafe {nn.as_ref()})
        }
    }
}

use std::ptr::NonNull;

use bi_list::BiList;


fn main() {
    let mut bl = BiList::new();
    bl.push(5);
    bl.push(10);
    bl.push(15);
    bl.push(20);

    
//    println!("{:#?}", bl);

    {
        //forward
        let mut it = bl.get_begin().unwrap();
        let end = NonNull::from(bl.get_end().unwrap());
        
        let mut c: u32 = 0; // to choose direction

        while {
            println!("val: {}", it.get_val());
            NonNull::from(it) != end
        } {
            match c % 3 {
                0 | 1 => {
                    it = it.next();
                },
                2 => {
                    it = it.prev();
                },
                _ => {
                    unreachable!();
                }
            }
            c += 1;
        }
    }
}
