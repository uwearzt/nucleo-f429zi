// ------------------------------------------------------------------------------
// Copyright 2018 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

#![no_main]
#![no_std]

// use smoltcp::socket::UdpSocket;
// use smoltcp::socket::UdpSocketBuffer;
// use smoltcp::socket::TcpSocketBuffer;

extern crate cortex_m;
use cortex_m::asm;

extern crate cortex_m_rt as rt;
use rt::entry;
use rt::exception;

extern crate panic_semihosting;
extern crate stm32f429;

use rt::ExceptionFrame;

extern crate embedded_hal;

#[entry]
fn main() -> ! {
    
    let peripherals = stm32f429::Peripherals::take().unwrap();
    let gpiob = &peripherals.GPIOB;
    let rcc = &peripherals.RCC;

    rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());
    rcc.apb1enr.modify(|_, w| w.tim7en().set_bit());

    // LEDs 1-3
    gpiob.moder.modify(|_, w| unsafe { w.moder0().bits(1) } );
    gpiob.moder.modify(|_, w| unsafe { w.moder7().bits(1) } );
    gpiob.moder.modify(|_, w| unsafe { w.moder14().bits(1) } );

    //let udp_rx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY], vec![0; 64]);
    //let udp_tx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY], vec![0; 128]);
    //let udp_socket = UdpSocket::new(udp_rx_buffer, udp_tx_buffer);
   
    loop {
        gpiob.bsrr.write(|w| w.bs0().set_bit());
        gpiob.bsrr.write(|w| w.bs7().set_bit());
        gpiob.bsrr.write(|w| w.bs14().set_bit());
        delay();

        gpiob.bsrr.write(|w| w.br0().set_bit());
        gpiob.bsrr.write(|w| w.br7().set_bit());
        gpiob.bsrr.write(|w| w.br14().set_bit());
        delay();
    }
}

fn delay() {
    for _i in 1..30000 {
        asm::nop();
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

