use ruboy;
use ruboy::cartridge::Cartridge;
use ruboy::cpu;
use ruboy::memory::Mmu;

#[test]
fn test_blargg_cpu_instrs() {
    let cartridge = Cartridge::new("rom/blargg/cpu_instrs.gb");

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_result_ok(&mmu);
}

#[test]
fn test_blargg_cpu_instrs_01() {
    let cartridge = Cartridge::new("rom/blargg/01-special.gb");

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_result_ok(&mmu);
}

#[test]
fn test_blargg_cpu_instrs_02() {
    let cartridge = Cartridge::new("rom/blargg/02-interrupts.gb");

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_result_ok(&mmu);
}

#[test]
fn test_blargg_cpu_instrs_03() {
    let cartridge = Cartridge::new("rom/blargg/03-op sp,hl.gb");

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_result_ok(&mmu);
}

#[test]
fn test_blargg_cpu_instrs_04() {
    let cartridge = Cartridge::new("rom/blargg/04-op r,imm.gb");

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_result_ok(&mmu);
}

#[test]
fn test_blargg_cpu_instrs_05() {
    let cartridge = Cartridge::new("rom/blargg/05-op rp.gb");

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_result_ok(&mmu);
}

#[test]
fn test_blargg_cpu_instrs_06() {
    let cartridge = Cartridge::new("rom/blargg/06-ld r,r.gb");

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_result_ok(&mmu);
}

#[test]
fn test_blargg_cpu_instrs_07() {
    let cartridge = Cartridge::new("rom/blargg/07-jr,jp,call,ret,rst.gb");

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_result_ok(&mmu);
}

#[test]
fn test_blargg_cpu_instrs_08() {
    let cartridge = Cartridge::new("rom/blargg/08-misc instrs.gb");

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_result_ok(&mmu);
}

#[test]
fn test_blargg_cpu_instrs_09() {
    let cartridge = Cartridge::new("rom/blargg/09-op r,r.gb");

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_result_ok(&mmu);
}

#[test]
fn test_blargg_cpu_instrs_10() {
    let cartridge = Cartridge::new("rom/blargg/10-bit ops.gb");

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_result_ok(&mmu);
}

#[test]
fn test_blargg_cpu_instrs_11() {
    let cartridge = Cartridge::new("rom/blargg/11-op a,(hl).gb");

    let mut cpu = cpu::init_cpu();
    let mut mmu = Mmu::new(cartridge);

    cpu.run(&mut mmu);

    assert_result_ok(&mmu);
}

fn assert_result_ok(mmu: &Mmu) {
    let output = get_screen_text(mmu);

    assert_eq!(false, output.contains("Failed"),
               "Expected screen output not to contain the word 'Failed'. Screen output:\n---\n{}\n---\n", output.as_str()
    );
}

fn get_screen_text(mmu: &Mmu) -> String {
    let mut output = "".to_owned();

    for tile in 0x9800..0x9BFF {
        let c = mmu[tile];
        if c != 0x00 {
            output.push(c as char);
        }
    }
    return output;
}
