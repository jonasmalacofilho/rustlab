use std::cell::RefCell;
use std::rc::Rc;

pub struct Vertex {
    data: i32,
    adj: Vec<Arc>,
}

pub struct Arc {
    data: i32,
    points_to: Rc<RefCell<Vertex>>,
}

impl Vertex {
    pub fn new(data: i32) -> Self {
        Vertex { data, adj: vec![] }
    }

    pub fn link_to(&mut self, other: Rc<RefCell<Vertex>>, data: i32) {
        let arc = Arc::new(data, other);
        self.adj.push(arc);
    }

    pub fn data(&self) -> i32 {
        self.data
    }

    pub fn set_data(&mut self, data: i32) -> i32 {
        self.data = data;
        self.data()
    }

    pub fn adj(&self) -> impl Iterator<Item = &Arc> {
        self.adj.iter()
    }
}

impl Arc {
    pub fn new(data: i32, points_to: Rc<RefCell<Vertex>>) -> Self {
        Arc { data, points_to }
    }

    pub fn data(&self) -> i32 {
        self.data
    }

    pub fn set_data(&mut self, data: i32) -> i32 {
        self.data = data;
        self.data()
    }

    pub fn adj(&self) -> &Rc<RefCell<Vertex>> {
        &self.points_to
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v1 = Rc::new(RefCell::new(Vertex::new(1)));
        let v2 = Rc::new(RefCell::new(Vertex::new(2)));

        assert_eq!(v1.borrow().data(), 1);
        assert_eq!(v2.borrow().data(), 2);

        v1.borrow_mut().link_to(Rc::clone(&v2), 42);
        v2.borrow_mut().link_to(Rc::clone(&v1), 40);

        {
            let v1 = v1.borrow();
            for arc in v1.adj() {
                let mut vadj = arc.adj().borrow_mut();
                vadj.set_data(v1.data() + arc.data());
            }

            assert_eq!(v2.borrow().data(), 43);
        }
    }
}
