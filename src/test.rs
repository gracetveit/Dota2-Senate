use crate::{Solution, Member, Allegience, Senate};

#[test]
fn example_1(){
    assert_eq!(Solution::predict_party_victory(String::from("RD")), "Radiant")
}

#[test]
fn example_2(){
    assert_eq!(Solution::predict_party_victory(String::from("RDD")), "Dire")
}

#[test]
fn new_member(){
    let new_member = Member::new(String::from("D"));
    assert!(matches!(new_member.allegience, Allegience::Dire))
}

#[test]
fn new_senate(){
    let new_senate = Senate::new(String::from("DRRRRDDDR"));
    assert!(matches!(new_senate.queue[0].allegience, Allegience::Dire))
}

#[test]
fn ban_member(){
    let mut new_senate = Senate::new(String::from("RDRR"));
    new_senate.ban_next_member(Allegience::Dire);
    assert_eq!("RRR", new_senate.print());
}

#[test]
fn evaluate(){
    let new_senate = Senate::new(String::from("DDRR"));
    assert_eq!(new_senate.evaluate(&Allegience::Dire), 0.5)
}
