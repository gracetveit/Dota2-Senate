use std::collections::VecDeque;

#[cfg(test)]
mod test;

#[derive(PartialEq)]
pub enum Allegience {
    Radiant,
    Dire
}

pub struct Member {
    pub allegience: Allegience,
}

impl Member {
    pub fn new(allegience: String) -> Self {
        if allegience == "D" {
            Member {
                allegience: Allegience::Dire,
            }
        } else if allegience == "R" {
            Member {
                allegience: Allegience::Radiant,
            }
        } else {panic!("Allegience must be set to 'R' or 'D'")}
    }
}

pub struct Senate {
    /// Use queue.pop for dequeueing, and queue.insert for enqueing
    queue: VecDeque<Member>
}

impl Senate {
    pub fn new(senate_str: String) -> Self {
        let mut queue: VecDeque<Member> = VecDeque::new();
        for c in senate_str.chars() {
            queue.push_back(Member::new(String::from(c)))
        }
        Senate {
            queue
        }
    }

    pub fn dequeue_and_requeue(&mut self) {
        let shuffle = self.queue.pop_front().expect("No Member");
        self.queue.push_back(shuffle);
    }

    pub fn ban_next_member(&mut self, allegience: Allegience) {
        for i in 0..self.queue.len() {
            let member = &self.queue[i];
            if member.allegience == allegience {
                self.queue.remove(i);
                break
            }
        }
    }

    pub fn print(&self) -> String {
        let mut string = String::new();
        for x in &self.queue {
            match x.allegience {
                Allegience::Dire => string.push('D'),
                Allegience::Radiant => string.push('R')
            }
        }
        string
    }

    pub fn evaluate(&self, allegience: &Allegience) -> f32 {
        let mut count: f32 = 0.0;

        for x in &self.queue {
            if &x.allegience == allegience {
                count += 1.0;
            }
        }

        count / self.queue.len() as f32
    }

    pub fn current_member(&self) -> &Member {
        &self.queue[0]
    }

}

pub struct Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut  parsed_senate = Senate::new(senate);
        let mut victory = false;
        let mut victor: String = String::from("No Victor");
        while !victory {
            let member = parsed_senate.current_member();
            let odds = parsed_senate.evaluate(&member.allegience);
            if odds >= 0.75 {
                victory = true;
                match member.allegience {
                    Allegience::Dire => victor = String::from("Dire"),
                    Allegience::Radiant => victor = String::from("Radiant")
                }
                // break;
            } else {
                match member.allegience {
                    Allegience::Dire => parsed_senate.ban_next_member(Allegience::Radiant),
                    Allegience::Radiant => parsed_senate.ban_next_member(Allegience::Dire)
                }
            }
            parsed_senate.dequeue_and_requeue();
        }

        victor
    }
}
