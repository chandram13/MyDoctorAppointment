

use std::env::{args, Args}

fn firstFunction() {
    let mut myVar: Args = args();

    let first = myVar.nth(0).unwrap(); // use unwrap to get self
    let myoperator = args.nth(2).unwrap().chars();.next().unwrap();
    let second = args.nth(3).unwrap()

    let first_name = first.parse::<f32>().unwrap();
    let last_name = second.parse::<f32>().unwrap()
    let result = operate(operator, first_name, last_name);

    println!("{:?}", result);
}



fn nth(&mut self, n: usize) -> Option<String> {
    doctorChoice = ["Doctor 1","Doctor 2","Doctor 3","Doctor 4"]
    self.inner.next() 
    self.inner.next() 
    self.inner.next()
    self.inner.next()
}

    // matching expression technique (Rust)
    fn operate(operator: String, first_name: str, last_name: str) -> str {
        match operator {
            '8:00' => first_name + last_name,
            '8:30' => first_name + last_name,
            '9:00' => first_name + last_name,
            '9:30' => first_name + last_name,
            '10:00' => first_name + last_name,
            '10:30' => first_name + last_name,
            '11:00' => first_name + last_name,
            '11:30' => first_name + last_name,
            '1:00' => first_name + last_name,
            '1:30' => first_name + last_name,
            '2:00' => first_name + last_name,
            '2:30' => first_name + last_name,
            '3:00' => first_name + last_name,
            '4:00' => first_name + last_name,
            '4:30' => first_name + last_name,
            '0:00' => panic!("Invalid time given.")
        }
        }

    struct NearbyHospital{
        givenbypatientAddress: givenbypatientAddress<u8>,
        givenbyZipcode: givenbyAddress<u8>,
        givenbyhospitalGroup: givenbyhospitalGroup<u8>,
        givenbypatientPlan: givenbypatientPlan<u8>,
        askpatientFacility: askpatientFacility<u8>
    }
    let nearby_Hospital = NearbyHospital {givenbypatientAddress: str, givenbyZipcode: f32, givenbyhospitalGroup: str, givenbypatientPlan: str, askpatientFacility: str};

    impl String {
        fn from(s: &str) -> Self{
            f32 {
                vec: Vec::from(s.phone_number())
                vec: Vec::from(s.height())
                vec: Vec::from(s.weight())
            }
        }
    }

    enum AskedQuestions{
        currentCondition,
        anyinfluenzaSymptoms,
        doespatientSmoke,
        doespatientDrink,
        takingMedications,
        requiredVaccinations,
        upcomingAppointment}
    
        fn currentAppointment -> Result<(), askedQuestions> {
            if currentCondition == "ok"{
                Err(AskedQuestions::("Maintain a healthy diet and exercise frequently.".to_string()))
            }
            else if currentConditon == "not satisfied"{
                Err(AskedQuestions::"What current symptoms are you feeling, is it physically or mental?".to_string())
            }
            if anyinfluenzaSymptoms == "None"{
                Err(AskedQuestions::"Maintain your health by not drinking or eating too much consuming cold food or substances.".to_string())
            }
            else if anyinfluenzaSymptoms == "Aware of"{
                Err(AskedQuestions::"Are you feeling the following: coughing, wheezing, feeling feverish, or sneezing?".to_string())
            }
            if doespatientSmoke == "No"{
                Err(AskedQuestions::"That's good news. Maintain your healthy by eating clean and regulate your sugar levels.".to_string())
            }
            else if doespatientSmoke == "Yes"{
                Err(AskedQuestions::"How often do you smoke? Do you characterize yourself as a casual or heavy smoker. Get help by quitting cold turkey and use nicotine patches.".to_string())
            }
            if doespatientDrink == "No"{
                Err(AskedQuestions::"That's good news. Maintain your healthy by eating clean and regulate your sugar levels.".to_string())
            }
            else if doespatientDrink == "Yes"{
                Err(AskedQuestions::"How often do you drink? Do you characterize yourself as a casual or heavy drinker? Drinking is normal in moderation, however, regulate your intake.".to_string())
            }
            if takingMedications = true{
                Err(AskedQuestions::"Are you taking your medications daily? Any irregular symptoms or other signs to be worried about?".to_string())
            }
            else if takingMedications = false{
                Err(AskedQuestions::"It's good to hear that you're healthy. Do you need to be prescribed anything for any current symptoms you might be experiecing. If not, I'll just measure your blood sugar levels and cholesterol.".to_string())
            }
            if requiredVaccinations == "Up to date"{
                Err(AskedQuestions::"Your vaccinations are all done, in doing so, you have prevented any known diseases to be spread.".to_string())
            }
            else if requiredVacinations == "Not up to date"{
                Err(AskedQuestions::"Your vaccinations are not all done, we will schedule an appointment or more to make sure you're vaccinated.".to_string())
            }
            if upcomingAppointment = true{
                Err(AskedQuestions::"There is an upcoming appointment listed on your records. Do you know the details of this appointment?".to_string())
            }
            else if upcomingAppointment = false{
                Err(AskedQuestions::"No appointments are coming up it appears. We will continue your appointment. Lastly, please make sure to maintain healthy habits".to_string())
            } else{
                Err(AskedQuestions::"The doctor is busy with another patient or there are too many patients causing a longer wait time.".to_string())
            }

        }





    fn output(first_name: String, operator: char, last_name: String, result: f32) -> String {
        format("{} {} {} = {}", first_name, operator, last_name})
        format("{} = {}",currentAppointment)
        println!("{:?}",output(first_name, operator, last_name, result));
        println!("{:?}",output(currentAppointment));
    }
}