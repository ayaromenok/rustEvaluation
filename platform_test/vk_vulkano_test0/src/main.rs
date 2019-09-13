use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;

fn main() {
    println!("Hello, Vulkano!");

    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("failed to create instance");

    let physical = PhysicalDevice::enumerate(&instance).next()
        .expect("no device available");

    for family in physical.queue_families(){
        println!("Found a queue famili with {:?} queue(s)", family.queues_count());
    }

}
