extern crate openai;
use openai::*;

extern crate rand;

struct RandomGymMember {}
impl GymMember for RandomGymMember {
   fn start (&mut self, s: &GymShape, mut t: &GymState) {}
   fn reward(&mut self, r: gym_reward, d: gym_done) {}
   fn tick(&mut self, screen: &mut Vec<u8>) -> gym_range {
      return vec![ ((rand::random::<u64>())%6) as usize ]
   }
   fn reset(&mut self) {}
   fn close(&mut self) {}
}

fn main() {
   let mut d = Gym::new();
   d.parse_args();
   d.start(|| { RandomGymMember{} });
}
