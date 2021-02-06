use crate::resources::{constants, ResourceDict};
use crate::ui::rand;
use crate::{object::Object, resources::ResourceID};
const MAX_GROWTH: f64 = 0.10;
impl Object {
    pub fn tick(&mut self, rss: &ResourceDict) {
        self.past = self.resources.clone(); //"backs up" the current resource amount
        self.resources.tick(); //does a tick of the resources

        self.grow_pops(); //Pop growth
        self.grow_plants(); //Plant growth

        self.pop_upkeep(); //Pop upkeep
        self.plant_upkeep(); //Plant upkeep

        self.pop_benefits(); //Pop benefits
        self.plant_benefits(); //Plant benefit
    }
    pub fn grow_pops(&mut self) {
        let pops: u64 = self.resources.get_curr(constants::POPULATION); //Gets current population
        if pops == 0 {
            return;
        } //If there's no population, don't do any of this
        let housing: u64 = self.resources.get_cap(constants::POPULATION); //Population capacity
        let growth: f64 = (pops as f64) * (1.0 + MAX_GROWTH * ((housing as f64 - pops as f64) / housing as f64) as f64); //Amount of projected growth
        let grown: u64 = rand::rand_round(growth); //Amount grown; rounded
        self.resources.change_amt(constants::POPULATION, grown); //Finalizes changes
    } //Grows the population.
    pub fn pop_benefits(&mut self) {
        let pops: u64 = self.resources.get_curr(constants::POPULATION); //Each unit of population...
        self.resources.add_res(constants::PRODUCTION, pops); //Produces 1
                                                             // production
    } //The population's benefits.
    pub const UPKEEP_RSS_POPS: &'static [(ResourceID, f64)] = &[(constants::AIR, 1.0), (constants::WATER, 1.0), (constants::FOOD, 1.0)];
    pub fn pop_upkeep(&mut self) -> bool {
        let mut satisfied: bool = true;
        for (rss, fct) in Object::UPKEEP_RSS_POPS {
            //For each bit of upkeep...
            let upkeep = rand::rand_round(self.resources.get_curr(constants::POPULATION) as f64 * fct); //Gets the amount of upkeep required
            if !self.resources.rmv_res(*rss, upkeep) {
                //If we don't have enough of the resource, and can't afford all of our
                // upkeep...
                let bottleneck = self.resources.get_curr(*rss); //Finds the amount we can still support
                self.resources.rmv_res_force(*rss, upkeep); //Gets rid of all of the resource required for upkeep
                self.resources.change_amt(constants::POPULATION, bottleneck); //Kills all of the things we can't support
                satisfied = false; //We aren't satisfied
            }
        }
        satisfied
    } //The population's upkeep

    pub fn grow_plants(&mut self) {
        let pops: u64 = self.resources.get_curr(constants::BIOMASS);
        if pops == 0 {
            return;
        }
        let housing: u64 = self.resources.get_cap(constants::BIOMASS);
        let growth: f64 = (pops as f64) * (1.0 + MAX_GROWTH * ((housing as f64 - pops as f64) / housing as f64) as f64);
        let grown: u64 = rand::rand_round(growth);
        self.resources.change_amt(constants::BIOMASS, grown);
    } //Same as grow_pops
    pub fn plant_benefits(&mut self) {
        let pops: u64 = self.resources.get_curr(constants::BIOMASS);
        self.resources.add_res(constants::AIR, rand::rand_round(pops as f64 / 10.0));
    } //Same as pop_benefits
    pub const UPKEEP_RSS_PLANTS: &'static [(ResourceID, f64)] = &[(constants::WATER, 0.05)];
    pub fn plant_upkeep(&mut self) -> bool {
        let mut satisfied: bool = true;
        for (rss, fct) in Object::UPKEEP_RSS_PLANTS {
            //For each bit of upkeep...
            let upkeep = rand::rand_round(self.resources.get_curr(constants::BIOMASS) as f64 * fct); //Gets the amount of upkeep required
            if !self.resources.rmv_res(*rss, upkeep) {
                //If we don't have enough of the resource, and can't afford all of our
                // upkeep...
                let bottleneck = self.resources.get_curr(*rss); //Finds the amount we can still support
                self.resources.rmv_res_force(*rss, upkeep); //Gets rid of all of the resource required for upkeep
                self.resources.change_amt(constants::BIOMASS, bottleneck); //Kills all of the things we can't support
                satisfied = false; //We aren't satisfied
            }
        }
        satisfied
    } //Plant upkeep
}
