pub trait ElectionDataService {
    fn create_election();

    fn start_election();

    fn stop_election();

    fn create_participant();

    fn vote();

    fn get_election_list();
}

trait UserLocationService {
    fn submit_location();


}

trait LocationDataService {
    fn create_region();

    fn set_region_name();

    fn set_coordinates();

}