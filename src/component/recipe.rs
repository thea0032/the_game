use crate::{
    extra_bits,
    resources::{constants as rscnst, ResourceDict, ResourceID},
};

#[derive(Clone, Debug)]
pub struct Recipe {
    cost: Vec<i64>,
}
impl Recipe {
    pub fn new(len: usize) -> Recipe {
        Recipe {
            cost: extra_bits::fill(len, 0),
        }
    }
    pub fn cost(&mut self) -> &mut Vec<i64> {
        &mut self.cost
    }
    pub fn cost_stat(&self) -> &Vec<i64> {
        &self.cost
    }
    pub fn display(&self, rss: &ResourceDict) -> String {
        let mut positives = Vec::new();
        let mut negatives = Vec::new();
        for (i, item) in self.cost.iter().enumerate() {
            match item.cmp(&0) {
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => {
                    positives.push(i);
                }
                std::cmp::Ordering::Less => {
                    negatives.push(i);
                }
            }
        }
        let mut res: String = "".to_string();
        let p_len = positives.len();
        let n_len = negatives.len();
        if p_len != 0 {
            res.push_str("Costs: ");
            for line in positives {
                res.push_str(&self.cost[line].to_string());
                res.push(' ');
                res.push_str(&rss.get(ResourceID::new(line)));
                res.push(',');
            }
            res.pop();
            res.push(' ');
            res.push('\n');
        }
        if n_len != 0 {
            res.push_str("Gains: ");
            for line in negatives {
                res.push_str(&(-self.cost[line]).to_string());
                res.push(' ');
                res.push_str(&rss.get(ResourceID::new(line)));
                res.push(',');
            }
            res.pop();
            res.push(' ');
        }
        if p_len == 0 && n_len == 0 {
            res.push_str("Empty recipe");
        }
        res
    }
}
pub fn get_all(rss: &ResourceDict) -> Vec<Recipe> {
    let mut res: Vec<Recipe> = Vec::new();
    res.push(increase_transfer_cap(rss));
    res.push(get_miner(rss));
    res.push(get_e_miner(rss));
    res.push(get_factory_worker_smelt(rss));
    res.push(get_factory_worker_mnfg(rss));
    res.push(get_farmer(rss));
    res.push(get_eff_farmer(rss));
    res
} //grabs all recipes
pub fn get_names() -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    res.push("Transfer capacity increase".to_string());
    res.push("Mine ore".to_string());
    res.push("Mine uranium".to_string());
    res.push("Smelt ore".to_string());
    res.push("Manufacture luxuries".to_string());
    res.push("Farm".to_string());
    res.push("Farm less efficiently to save biomass".to_string());
    res
} //grabs all recipe names
pub fn increase_transfer_cap(rss: &ResourceDict) -> Recipe {
    let mut res = Recipe::new(rss.len());
    let r_cost = res.cost();
    r_cost[rscnst::PRODUCTION.get()] = 1;
    r_cost[rscnst::TRANSFER.get()] = -10;
    res
}
/*
pub fn get_generic(rss:&ResourceDict) -> Recipe{
    let mut res = Recipe::new(rss.len());
    let r_cost = res.cost();

    res
}//Allows me to easily copy and paste to make new methods
*/
pub fn get_miner(rss: &ResourceDict) -> Recipe {
    //sample recipe:
    let mut res = Recipe::new(rss.len()); //Initializes result
    let r_cost = res.cost(); //Gets cost vector directly, to make everything cleaner
    r_cost[rscnst::ORE.get()] = -10; //Gives 10 ore
    r_cost[rscnst::ENERGY.get()] = 1; //Costs 1 energy
    r_cost[rscnst::PRODUCTION.get()] = 1; //Costs 1 production
    r_cost[rscnst::MINING_JOBS.get()] = 1; //Costs 1 mining job
    res //Result is returned
}
pub fn get_e_miner(rss: &ResourceDict) -> Recipe {
    let mut res = Recipe::new(rss.len());
    let r_cost = res.cost();
    r_cost[rscnst::URANIUM.get()] = -10;
    r_cost[rscnst::ENERGY.get()] = 1;
    r_cost[rscnst::PRODUCTION.get()] = 1;
    r_cost[rscnst::URANIUM_MINING_JOBS.get()] = 1;
    res
}
pub fn get_farmer(rss: &ResourceDict) -> Recipe {
    let mut res = Recipe::new(rss.len());
    let r_cost = res.cost();
    r_cost[rscnst::FOOD.get()] = -20;
    r_cost[rscnst::ENERGY.get()] = 1;
    r_cost[rscnst::BIOMASS.get()] = 20;
    r_cost[rscnst::PRODUCTION.get()] = 1;
    res
}
pub fn get_eff_farmer(rss: &ResourceDict) -> Recipe {
    let mut res = Recipe::new(rss.len());
    let r_cost = res.cost();
    r_cost[rscnst::FOOD.get()] = -10;
    r_cost[rscnst::ENERGY.get()] = 1;
    r_cost[rscnst::BIOMASS.get()] = 5;
    r_cost[rscnst::PRODUCTION.get()] = 1;
    res
}
pub fn get_factory_worker_smelt(rss: &ResourceDict) -> Recipe {
    let mut res = Recipe::new(rss.len());
    let r_cost = res.cost();
    r_cost[rscnst::METAL.get()] = -40;
    r_cost[rscnst::ORE.get()] = 10;
    r_cost[rscnst::ENERGY.get()] = 3;
    r_cost[rscnst::PRODUCTION.get()] = 1;
    r_cost[rscnst::FACTORY_JOBS.get()] = 1;
    res
}
pub fn get_factory_worker_mnfg(rss: &ResourceDict) -> Recipe {
    let mut res = Recipe::new(rss.len());
    let r_cost = res.cost();
    r_cost[rscnst::LUXURIES.get()] = -20;
    r_cost[rscnst::METAL.get()] = 10;
    r_cost[rscnst::ENERGY.get()] = 3;
    r_cost[rscnst::PRODUCTION.get()] = 1;
    r_cost[rscnst::FACTORY_JOBS.get()] = 1;
    res
}
