pub struct PrintInfo {
    pub name: String,
    pub class: String,
    pub size: (usize, usize, usize),
    pub num_iter: i32,
    pub time: f64,
    pub mops: f64,
    pub operation: String,
    pub verified: i8,
    pub num_threads: u32,
    //pub uns: bool
}

pub fn printer(info: PrintInfo) {
    let (n1, n2, n3) = info.size;

    println!("\n\n {} Benchmark Completed\n", info.name);
    println!(" Class           =    {}", info.class);
    if n2 == 0 {
        println!(" Size            =    {}", n1);
    } else {
        println!(" Size            =    {} x {} x {}", n1, n2, n3);
    }
    println!(" Total threads   =    {}", info.num_threads);
    println!(" Iterations      =    {}", info.num_iter);
    println!(" Time in seconds =    {:.4}", info.time);
    println!(" Mop/s total     =    {}", info.mops);
    println!(" Operation type  =    {}", info.operation);

    if info.verified < 0 {
        println!(" Verification    =    NOT PERFORMED");
    } else if info.verified == 1 {
        println!(" Verification    =    SUCCESSFUL");
    } else {
        println!(" Verification    =    UNSUCCESSFUL");
    }
    println!(" Version         =    0.0.1");
    println!(" RAND            =    randdp");

    /*/
    if !info.uns {
        println!(" unsafe          =    disabled");
    } else {
        println!(" unsafe          =    enabled");
    }
    */

    /*
     * printf(" Please send the results of this run to:\n\n");
     * printf(" NPB Development Team\n");
     * printf(" Internet: npb@nas.nasa.gov\n \n");
     * printf(" If email is not available, send this to:\n\n");
     * printf(" MS T27A-1\n");
     * printf(" NASA Ames Research Center\n");
     * printf(" Moffett Field, CA  94035-1000\n\n");
     * printf(" Fax: 650-604-3957\n\n");
     */
    println!("\n\n");

    println!("----------------------------------------------------------------------");
    println!("    NPB-Rust is developed by: ");
    println!("        Eduardo Machado Martins");
    println!("        Leonardo Gibrowski Fae");
    println!("        Renato Barreto Hoffmann Filho");
    println!("        Lucas Sperhacke Bianchessi");
    println!("        Dalvan Griebler");
    println!();
    println!("    In case of questions or problems, please send an e-mail to us:");
    println!("        dalvan.griebler@edu.pucrs.br");
    println!("----------------------------------------------------------------------");
    println!();
}