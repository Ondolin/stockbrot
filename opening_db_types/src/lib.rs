use rand::Rng;

pub type BuildNodeCount = (u16, BuildNode);

pub type NodeCount = (u16, Node);

#[derive(Eq, Clone, Ord, PartialOrd, Debug)]
pub struct BuildNode {
    pub joice: String,
    pub children: Vec<BuildNodeCount>
}

#[derive(Eq, Clone, Ord, PartialOrd, Debug)]
pub struct Node {
    pub joice: &'static str,
    pub children: &'static [NodeCount]
}

impl PartialEq for BuildNode {
    fn eq(&self, other: &Self) -> bool {
        self.joice == other.joice
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.joice == other.joice
    }
}

impl BuildNode {
    pub fn new(move_name: String) -> BuildNode {
        BuildNode {
            joice: move_name,
            children: vec![]
        }
    }

    pub fn add_child(&mut self, child: BuildNode) -> &mut BuildNode {

        let index = if let Some(index) = self.children.iter().position(|c| c.1 == child) {
            self.children[index].0 += 1;
            index
        } else {
            self.children.push((1, child.clone()));
            self.children.iter().position(|c| c.1 == child).unwrap()
        };

        return &mut self.children[index].1;
    }
}

impl Node {
    pub fn get_best_node(&self) -> Option<&NodeCount> {
        let best = self.children.iter().filter(|a| a.0 >= 2).collect::<Vec<&NodeCount>>();
        if best.len() == 0 { return None }

        let mut games_in_best = 0;
        for game in &best {
            games_in_best += game.0;
        }

        let mut rng = rand::thread_rng();
        let mut random_game = rng.gen_range(0..games_in_best);

        for game in best {
            random_game -= game.0;

            if random_game <= 0 {
                return Some(game);
            }
        }

        unreachable!()

    }

    pub fn get_best_move(&self) -> Option<String> {
        let Some(joice) = self.children.iter().max() else { return None };

        Some(joice.1.joice.to_string())
    }

    pub fn get_node_by_move(&self, joice: String) -> Option<Node> {
        for node in self.children {
            if node.1.joice == joice {
                return Some(node.1.clone());
            }
        }
        None
    }

}