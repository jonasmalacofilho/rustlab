#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(transparent)]
pub struct VertexId {
    inner: usize,
}

impl VertexId {
    pub fn new(id: usize) -> Self {
        VertexId { inner: id }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(transparent)]
pub struct ArcId {
    inner: usize,
}

impl ArcId {
    pub fn new(id: usize) -> Self {
        ArcId { inner: id }
    }
}

#[derive(Debug)]
struct Vertex<C> {
    id: VertexId,
    parent: Option<(VertexId, C)>,
    adj: Vec<Arc<C>>,
}

impl<C> Vertex<C> {
    fn new(id: VertexId) -> Self {
        Vertex {
            id,
            parent: None,
            adj: vec![],
        }
    }
}

#[derive(Debug)]
struct Arc<C> {
    id: ArcId,
    cost: C,
    points_to: VertexId,
}

impl<C> Arc<C> {
    fn new(id: ArcId, cost: C, points_to: VertexId) -> Self {
        Arc {
            id,
            cost,
            points_to,
        }
    }
}

#[derive(Debug)]
pub struct Graph<C> {
    vertices: Vec<Vertex<C>>,
}

impl<C> Graph<C> {
    pub fn new() -> Self {
        Graph { vertices: vec![] }
    }

    pub fn add_vertex(&mut self) -> VertexId {
        let id = VertexId::new(self.vertices.len());
        self.vertices.push(Vertex::new(id));
        id
    }

    pub fn add_arc(&mut self, from: VertexId, to: VertexId, cost: C) -> ArcId {
        let _ = &self.vertices[to.inner];
        let from = &mut self.vertices[from.inner];

        let id = ArcId::new(from.adj.len());
        from.adj.push(Arc::new(id, cost, to));

        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut graph: Graph<i32> = Graph::new();

        let v1 = graph.add_vertex();
        let v2 = graph.add_vertex();
        let v3 = graph.add_vertex();
        let v4 = graph.add_vertex();

        graph.add_arc(v1, v2, 42);
        graph.add_arc(v1, v3, 69);
        graph.add_arc(v2, v3, 40);
        graph.add_arc(v2, v4, 32);
        graph.add_arc(v3, v4, 20);

        // {
        //     assert_eq!(v2.parent(), Some((v1.id(), 42)));
        //     assert_eq!(v3.parent(), Some((v1.id(), 69)));
        //     assert_eq!(v4.parent(), Some((v2.id(), 74)));
        // }
    }
}
