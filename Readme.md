Run `cargo run --release -- 0x21000000` and check the contents of `target/release/bitvec-test` with a disassembler.

```asm
__ZN11bitvec_test10extract_2417h87f34eef95a8a12dE:        // bitvec_test::extract_24::h87f34eef95a8a12d
0000000100001984         sub        sp, sp, #0x20                               ; End of try block started at 0x10000190c, CODE XREF=__ZN11bitvec_test4main17h5a0da1820dd45e67E+236
0000000100001988         stp        x29, x30, [sp, #0x10]
000000010000198c         add        x29, sp, #0x10
0000000100001990         stur       w0, [x29, var_4]
0000000100001994         adr        x4, #0x100038408
0000000100001998         nop
000000010000199c         sub        x2, x29, #0x4
00000001000019a0         movz       w0, #0x8
00000001000019a4         movz       w1, #0x20
00000001000019a8         movz       w3, #0x100
00000001000019ac         bl         __ZN103_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$bitvec..slice..api..BitSliceIndex$LT$T$C$O$GT$$GT$5index17h6c965b5776c57107E ; _$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$bitvec..slice..api..BitSliceIndex$LT$T$C$O$GT$$GT$::index::h6c965b5776c57107
00000001000019b0         bl         __ZN98_$LT$bitvec..slice..BitSlice$LT$T$C$bitvec..order..Msb0$GT$$u20$as$u20$bitvec..field..BitField$GT$7load_le17h550deac445cb8f32E ; _$LT$bitvec..slice..BitSlice$LT$T$C$bitvec..order..Msb0$GT$$u20$as$u20$bitvec..field..BitField$GT$::load_le::h550deac445cb8f32
00000001000019b4         ldp        x29, x30, [sp, #0x10]
00000001000019b8         add        sp, sp, #0x20
00000001000019bc         ret
                        ; endp
```

```asm
__ZN103_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$bitvec..slice..api..BitSliceIndex$LT$T$C$O$GT$$GT$5index17h6c965b5776c57107E:        // _$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$bitvec..slice..api..BitSliceIndex$LT$T$C$O$GT$$GT$::index::h6c965b5776c57107
0000000100000f84         sub        sp, sp, #0x30                               ; CODE XREF=__ZN11bitvec_test10extract_2417h87f34eef95a8a12dE+40
0000000100000f88         stp        x29, x30, [sp, #0x20]
0000000100000f8c         add        x29, sp, #0x20
0000000100000f90         stp        x0, x1, [sp, #0x8]
0000000100000f94         lsr        x8, x3, #0x3
0000000100000f98         stur       x8, [x29, var_8]
0000000100000f9c         cmp        x8, x0
0000000100000fa0         ccmp       x8, x1, #0x0
0000000100000fa4         ccmp       x1, x0, #0x0
0000000100000fa8         b.lo       loc_100000fe8

0000000100000fac         and        x9, x2, #0xfffffffffffffffc
0000000100000fb0         lsl        w8, w2, #0x3
0000000100000fb4         and        x8, x8, #0x18
0000000100000fb8         bfxil      x8, x3, #0x0, #0x3
0000000100000fbc         add        x8, x8, x0
0000000100000fc0         asr        x10, x8, #0x5
0000000100000fc4         add        x9, x9, x10, lsl #2
0000000100000fc8         sub        x10, x1, x0
0000000100000fcc         bfxil      x9, x8, #0x3, #0x2
0000000100000fd0         bfi        x8, x10, #0x3, #0x3d
0000000100000fd4         mov        x0, x9
0000000100000fd8         mov        x1, x8
0000000100000fdc         ldp        x29, x30, [sp, #0x20]
0000000100000fe0         add        sp, sp, #0x30
0000000100000fe4         ret
                        ; endp

                     loc_100000fe8:
0000000100000fe8         add        x0, sp, #0x8                                ; CODE XREF=__ZN103_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$bitvec..slice..api..BitSliceIndex$LT$T$C$O$GT$$GT$5index17h6c965b5776c57107E+36
0000000100000fec         sub        x1, x29, #0x8
0000000100000ff0         bl         __ZN103_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$bitvec..slice..api..BitSliceIndex$LT$T$C$O$GT$$GT$5index28_$u7b$$u7b$closure$u7d$$u7d$17hcfc4df3833ca0440E ; _$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$bitvec..slice..api..BitSliceIndex$LT$T$C$O$GT$$GT$::index::_$u7b$$u7b$closure$u7d$$u7d$::hcfc4df3833ca0440
                     __ZN103_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$bitvec..slice..api..BitSliceIndex$LT$T$C$O$GT$$GT$5index28_$u7b$$u7b$closure$u7d$$u7d$17hcfc4df3833ca0440E:        // _$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$bitvec..slice..api..BitSliceIndex$LT$T$C$O$GT$$GT$::index::_$u7b$$u7b$closure$u7d$$u7d$::hcfc4df3833ca0440
0000000100000ff4         sub        sp, sp, #0x60                               ; CODE XREF=__ZN103_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$bitvec..slice..api..BitSliceIndex$LT$T$C$O$GT$$GT$5index17h6c965b5776c57107E+108
0000000100000ff8         stp        x29, x30, [sp, #0x50]
0000000100000ffc         add        x29, sp, #0x50
0000000100001000         adr        x8, #0x100001044
0000000100001004         nop
0000000100001008         stp        x0, x8, [x29, var_20]
000000010000100c         adr        x8, #0x10002e850
0000000100001010         nop
0000000100001014         stp        x1, x8, [x29, var_10]
0000000100001018         adr        x8, #0x100038220
000000010000101c         nop
0000000100001020         movz       w9, #0x2
0000000100001024         stp        x8, x9, [sp]
0000000100001028         stp        xzr, xzr, [sp, #0x10]
000000010000102c         sub        x8, x29, #0x20
0000000100001030         stp        x8, x9, [sp, #0x20]
0000000100001034         adr        x1, #0x100038240
0000000100001038         nop
000000010000103c         mov        x0, sp
0000000100001040         bl         __ZN4core9panicking9panic_fmt17h9c91c97087ee8166E ; core::panicking::panic_fmt::h9c91c97087ee8166
                     __ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE:        // _$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$::fmt::hb7889bc78f650b2f
0000000100001044         sub        sp, sp, #0x50                               ; DATA XREF=__ZN103_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$bitvec..slice..api..BitSliceIndex$LT$T$C$O$GT$$GT$5index28_$u7b$$u7b$closure$u7d$$u7d$17hcfc4df3833ca0440E+12
0000000100001048         stp        x20, x19, [sp, #0x30]
000000010000104c         stp        x29, x30, [sp, #0x40]
0000000100001050         add        x29, sp, #0x40
0000000100001054         mov        x19, x1
0000000100001058         mov        x20, x0
000000010000105c         mov        x0, x1
0000000100001060         bl         __ZN4core3fmt9Formatter15debug_lower_hex17h696a970b385109a7E ; core::fmt::Formatter::debug_lower_hex::h696a970b385109a7
0000000100001064         tbz        w0, 0x0, loc_10000107c

0000000100001068         mov        x0, x20
000000010000106c         mov        x1, x19
0000000100001070         bl         __ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17h2e1ddcf7b3f71871E ; core::fmt::num::_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$::fmt::h2e1ddcf7b3f71871
0000000100001074         tbz        w0, 0x0, loc_1000010ac

0000000100001078         b          loc_1000010fc

                     loc_10000107c:
000000010000107c         mov        x0, x19                                     ; CODE XREF=__ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+32
0000000100001080         bl         __ZN4core3fmt9Formatter15debug_upper_hex17hd4c7dea0cf8d9038E ; core::fmt::Formatter::debug_upper_hex::hd4c7dea0cf8d9038
0000000100001084         tbz        w0, 0x0, loc_10000109c

0000000100001088         mov        x0, x20
000000010000108c         mov        x1, x19
0000000100001090         bl         __ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h6b04a9220a8999fcE ; core::fmt::num::_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$::fmt::h6b04a9220a8999fc
0000000100001094         tbz        w0, 0x0, loc_1000010ac

0000000100001098         b          loc_1000010fc

                     loc_10000109c:
000000010000109c         mov        x0, x20                                     ; CODE XREF=__ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+64
00000001000010a0         mov        x1, x19
00000001000010a4         bl         __ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h5761eaedf6dd4d41E ; core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$::fmt::h5761eaedf6dd4d41
00000001000010a8         tbnz       w0, 0x0, loc_1000010fc

                     loc_1000010ac:
00000001000010ac         adr        x8, #0x100038258                            ; CODE XREF=__ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+48, __ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+80
00000001000010b0         nop
00000001000010b4         movz       w9, #0x1
00000001000010b8         stp        x8, x9, [sp]
00000001000010bc         stp        xzr, xzr, [sp, #0x10]
00000001000010c0         adr        x8, #0x100031b80
00000001000010c4         nop
00000001000010c8         stp        x8, xzr, [sp, #0x20]
00000001000010cc         mov        x1, sp
00000001000010d0         mov        x0, x19
00000001000010d4         bl         __ZN4core3fmt9Formatter9write_fmt17h81a37fa83771ea92E ; core::fmt::Formatter::write_fmt::h81a37fa83771ea92
00000001000010d8         cbnz       w0, loc_1000010fc

00000001000010dc         add        x20, x20, #0x8
00000001000010e0         mov        x0, x19
00000001000010e4         bl         __ZN4core3fmt9Formatter15debug_lower_hex17h696a970b385109a7E ; core::fmt::Formatter::debug_lower_hex::h696a970b385109a7
00000001000010e8         tbz        w0, 0x0, loc_100001110

00000001000010ec         mov        x0, x20
00000001000010f0         mov        x1, x19
00000001000010f4         bl         __ZN4core3fmt3num55_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$3fmt17h2e1ddcf7b3f71871E ; core::fmt::num::_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$usize$GT$::fmt::h2e1ddcf7b3f71871
00000001000010f8         tbz        w0, 0x0, loc_10000112c

                     loc_1000010fc:
00000001000010fc         movz       w0, #0x1                                    ; CODE XREF=__ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+52, __ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+84, __ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+100, __ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+148, __ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+228, __ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+268
0000000100001100         ldp        x29, x30, [sp, #0x40]
0000000100001104         ldp        x20, x19, [sp, #0x30]
0000000100001108         add        sp, sp, #0x50
000000010000110c         ret
                        ; endp

                     loc_100001110:
0000000100001110         mov        x0, x19                                     ; CODE XREF=__ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+164
0000000100001114         bl         __ZN4core3fmt9Formatter15debug_upper_hex17hd4c7dea0cf8d9038E ; core::fmt::Formatter::debug_upper_hex::hd4c7dea0cf8d9038
0000000100001118         tbz        w0, 0x0, loc_100001140

000000010000111c         mov        x0, x20
0000000100001120         mov        x1, x19
0000000100001124         bl         __ZN4core3fmt3num53_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$3fmt17h6b04a9220a8999fcE ; core::fmt::num::_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i64$GT$::fmt::h6b04a9220a8999fc
0000000100001128         tbnz       w0, 0x0, loc_1000010fc

                     loc_10000112c:
000000010000112c         movz       w0, #0x0                                    ; CODE XREF=__ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+180, __ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+264
0000000100001130         ldp        x29, x30, [sp, #0x40]
0000000100001134         ldp        x20, x19, [sp, #0x30]
0000000100001138         add        sp, sp, #0x50
000000010000113c         ret
                        ; endp

                     loc_100001140:
0000000100001140         mov        x0, x20                                     ; CODE XREF=__ZN71_$LT$core..ops..range..Range$LT$Idx$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb7889bc78f650b2fE+212
0000000100001144         mov        x1, x19
0000000100001148         bl         __ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$3fmt17h5761eaedf6dd4d41E ; core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$u64$GT$::fmt::h5761eaedf6dd4d41
000000010000114c         tbz        w0, 0x0, loc_10000112c

0000000100001150         b          loc_1000010fc
```

```asm
__ZN98_$LT$bitvec..slice..BitSlice$LT$T$C$bitvec..order..Msb0$GT$$u20$as$u20$bitvec..field..BitField$GT$7load_le17h550deac445cb8f32E:        // _$LT$bitvec..slice..BitSlice$LT$T$C$bitvec..order..Msb0$GT$$u20$as$u20$bitvec..field..BitField$GT$::load_le::h550deac445cb8f32

00000001000012ac         sub        sp, sp, #0xd0                               ; CODE XREF=__ZN11bitvec_test10extract_2417h87f34eef95a8a12dE+44
00000001000012b0         stp        x24, x23, [sp, #0x90]
00000001000012b4         stp        x22, x21, [sp, #0xa0]
00000001000012b8         stp        x20, x19, [sp, #0xb0]
00000001000012bc         stp        x29, x30, [sp, #0xc0]
00000001000012c0         add        x29, sp, #0xc0
00000001000012c4         lsr        x8, x1, #0x3
00000001000012c8         adr        x9, #0x100031c04
00000001000012cc         nop
00000001000012d0         movz       w10, #0x4
00000001000012d4         stp        x9, x10, [sp, #0x40]
00000001000012d8         str        x8, [sp, #0xc0 + var_70]
00000001000012dc         sub        x9, x8, #0x1
00000001000012e0         cmp        x9, #0x20
00000001000012e4         b.hs       loc_100001460

00000001000012e8         lsl        w9, w0, #0x3
00000001000012ec         and        x2, x9, #0x18
00000001000012f0         bfxil      x2, x1, #0x0, #0x3
00000001000012f4         add        x9, x2, x8
00000001000012f8         lsr        x10, x9, #0x5
00000001000012fc         tst        x9, #0x1f
0000000100001300         cinc       x9, x10, ne
0000000100001304         movz       w10, #0x20
0000000100001308         add        w11, w2, w8
000000010000130c         sub        x10, x10, x2
0000000100001310         sub        w12, w8, w10
0000000100001314         ands       w12, w12, #0x1f
0000000100001318         cset       w13, eq
000000010000131c         orr        w12, w12, w13, lsl #5
0000000100001320         cmp        x8, x10
0000000100001324         csel       w8, w11, w12, ls
0000000100001328         cmp        x1, #0x8
000000010000132c         csel       w3, w2, w8, lo
0000000100001330         and        x0, x0, #0xfffffffffffffffc
0000000100001334         adr        x8, #0x100000e94
0000000100001338         nop
000000010000133c         adr        x10, #0x100000e0c
0000000100001340         nop
0000000100001344         and        w11, w3, #0xff
0000000100001348         adr        x12, #0x100000eb4
000000010000134c         nop
0000000100001350         adr        x13, #0x100000f28
0000000100001354         nop
0000000100001358         cmp        x9, #0x1
000000010000135c         csel       x12, x13, x12, eq
0000000100001360         cmp        w11, #0x20
0000000100001364         adr        x11, #0x100000e58
0000000100001368         nop
000000010000136c         adr        x13, #0x100000f6c
0000000100001370         nop
0000000100001374         csel       x10, x10, x12, eq
0000000100001378         csel       x11, x13, x11, eq
000000010000137c         cmp        w2, #0x0
0000000100001380         csel       x10, x10, x11, ne
0000000100001384         cmp        x9, #0x0
0000000100001388         csel       x10, x8, x10, eq
000000010000138c         add        x8, sp, #0x8
0000000100001390         mov        x1, x9
0000000100001394         blr        x10
0000000100001398         ldr        x8, [sp, #0xc0 + var_B8]
000000010000139c         cmp        x8, #0x1
00000001000013a0         b.ne       loc_1000013d4

00000001000013a4         ldp        x19, x21, [sp, #0x10]
00000001000013a8         ldp        x22, x23, [sp, #0x20]
00000001000013ac         ldr        x0, [sp, #0xc0 + var_90]
00000001000013b0         cbz        x0, loc_1000013f4

00000001000013b4         ldr        x20, [sp, #0xc0 + var_88]
00000001000013b8         lsr        x8, x20, #0x28
00000001000013bc         neg        w24, w8
00000001000013c0         bl         __ZN47_$LT$u32$u20$as$u20$bitvec..store..BitStore$GT$10load_value17h83c8f3f5180cc60eE ; _$LT$u32$u20$as$u20$bitvec..store..BitStore$GT$::load_value::h83c8f3f5180cc60e
00000001000013c4         and        w8, w0, w20
00000001000013c8         lsr        w20, w8, w24
00000001000013cc         cbnz       x23, loc_1000013fc

00000001000013d0         b          loc_100001418

                     loc_1000013d4:
00000001000013d4         ldr        w19, [sp, #0xc0 + var_A8]                   ; CODE XREF=__ZN98_$LT$bitvec..slice..BitSlice$LT$T$C$bitvec..order..Msb0$GT$$u20$as$u20$bitvec..field..BitField$GT$7load_le17h550deac445cb8f32E+244
00000001000013d8         ldr        x0, [sp, #0xc0 + var_B0]
00000001000013dc         ldrb       w8, [sp, #0xc0 + var_A3]
00000001000013e0         neg        w20, w8
00000001000013e4         bl         __ZN47_$LT$u32$u20$as$u20$bitvec..store..BitStore$GT$10load_value17h83c8f3f5180cc60eE ; _$LT$u32$u20$as$u20$bitvec..store..BitStore$GT$::load_value::h83c8f3f5180cc60e
00000001000013e8         and        w8, w0, w19
00000001000013ec         lsr        w20, w8, w20
00000001000013f0         b          loc_100001444

                     loc_1000013f4:
00000001000013f4         movz       w20, #0x0                                   ; CODE XREF=__ZN98_$LT$bitvec..slice..BitSlice$LT$T$C$bitvec..order..Msb0$GT$$u20$as$u20$bitvec..field..BitField$GT$7load_le17h550deac445cb8f32E+260
00000001000013f8         cbz        x23, loc_100001418

                     loc_1000013fc:
00000001000013fc         lsl        x23, x23, #0x2                              ; CODE XREF=__ZN98_$LT$bitvec..slice..BitSlice$LT$T$C$bitvec..order..Msb0$GT$$u20$as$u20$bitvec..field..BitField$GT$7load_le17h550deac445cb8f32E+288
0000000100001400         sub        x22, x22, #0x4

                     loc_100001404:
0000000100001404         add        x0, x22, x23                                ; CODE XREF=__ZN98_$LT$bitvec..slice..BitSlice$LT$T$C$bitvec..order..Msb0$GT$$u20$as$u20$bitvec..field..BitField$GT$7load_le17h550deac445cb8f32E+360
0000000100001408         bl         __ZN47_$LT$u32$u20$as$u20$bitvec..store..BitStore$GT$10load_value17h83c8f3f5180cc60eE ; _$LT$u32$u20$as$u20$bitvec..store..BitStore$GT$::load_value::h83c8f3f5180cc60e
000000010000140c         orr        w20, w0, w20
0000000100001410         subs       x23, x23, #0x4
0000000100001414         b.ne       loc_100001404

                     loc_100001418:
0000000100001418         cbz        x19, loc_100001444                          ; CODE XREF=__ZN98_$LT$bitvec..slice..BitSlice$LT$T$C$bitvec..order..Msb0$GT$$u20$as$u20$bitvec..field..BitField$GT$7load_le17h550deac445cb8f32E+292, __ZN98_$LT$bitvec..slice..BitSlice$LT$T$C$bitvec..order..Msb0$GT$$u20$as$u20$bitvec..field..BitField$GT$7load_le17h550deac445cb8f32E+332

000000010000141c         lsr        x8, x21, #0x20
0000000100001420         movz       w9, #0x20
0000000100001424         sub        x8, x9, w8, uxtb
0000000100001428         cmp        x8, #0x20
000000010000142c         csel       w8, w8, wzr, lo
0000000100001430         lsl        w20, w20, w8
0000000100001434         mov        x0, x19
0000000100001438         bl         __ZN47_$LT$u32$u20$as$u20$bitvec..store..BitStore$GT$10load_value17h83c8f3f5180cc60eE ; _$LT$u32$u20$as$u20$bitvec..store..BitStore$GT$::load_value::h83c8f3f5180cc60e
000000010000143c         and        w8, w0, w21
0000000100001440         orr        w20, w8, w20

                     loc_100001444:
0000000100001444         mov        x0, x20                                     ; CODE XREF=__ZN98_$LT$bitvec..slice..BitSlice$LT$T$C$bitvec..order..Msb0$GT$$u20$as$u20$bitvec..field..BitField$GT$7load_le17h550deac445cb8f32E+324, loc_100001418
0000000100001448         ldp        x29, x30, [sp, #0xc0]
000000010000144c         ldp        x20, x19, [sp, #0xb0]
0000000100001450         ldp        x22, x21, [sp, #0xa0]
0000000100001454         ldp        x24, x23, [sp, #0x90]
0000000100001458         add        sp, sp, #0xd0
000000010000145c         ret
                        ; endp

                     loc_100001460:
0000000100001460         movz       w8, #0x20                                   ; CODE XREF=__ZN98_$LT$bitvec..slice..BitSlice$LT$T$C$bitvec..order..Msb0$GT$$u20$as$u20$bitvec..field..BitField$GT$7load_le17h550deac445cb8f32E+56
0000000100001464         stur       x8, [x29, var_38]
0000000100001468         add        x8, sp, #0x40
000000010000146c         adr        x9, #0x1000011d0
0000000100001470         nop
0000000100001474         stp        x8, x9, [sp, #0x8]
0000000100001478         sub        x8, x29, #0x38
000000010000147c         adr        x9, #0x10002e850
0000000100001480         nop
0000000100001484         stp        x8, x9, [sp, #0x18]
0000000100001488         add        x8, sp, #0x50
000000010000148c         stp        x8, x9, [sp, #0x28]
0000000100001490         adr        x8, #0x1000382d8
0000000100001494         nop
0000000100001498         movz       w9, #0x4
000000010000149c         stp        x8, x9, [sp, #0x58]
00000001000014a0         stp        xzr, xzr, [sp, #0x68]
00000001000014a4         add        x8, sp, #0x8
00000001000014a8         movz       w9, #0x3
00000001000014ac         stp        x8, x9, [sp, #0x78]
00000001000014b0         adr        x1, #0x100038318
00000001000014b4         nop
00000001000014b8         add        x0, sp, #0x58
00000001000014bc         bl         __ZN4core9panicking9panic_fmt17h9c91c97087ee8166E ; core::panicking::panic_fmt::h9c91c97087ee8166
                     __ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7c019c8f90d821d9E:        // core::ptr::drop_in_place$LT$std..env..Args$GT$::h7c019c8f90d821d9
00000001000014c0         stp        x22, x21, [sp, #-0x30]!                     ; CODE XREF=__ZN11bitvec_test4main17h5a0da1820dd45e67E+960, __ZN11bitvec_test4main17h5a0da1820dd45e67E+996
00000001000014c4         stp        x20, x19, [sp, #0x10]
00000001000014c8         stp        x29, x30, [sp, #0x20]
00000001000014cc         add        x29, sp, #0x20
00000001000014d0         mov        x19, x0
00000001000014d4         ldp        x8, x9, [x0, #0x10]
00000001000014d8         subs       x9, x9, x8
00000001000014dc         b.eq       loc_100001524

00000001000014e0         asr        x9, x9, #0x3
00000001000014e4         orr        x10, xzr, #0xaaaaaaaaaaaaaaaa
00000001000014e8         movk       x10, #0xaaab
00000001000014ec         mul        x9, x9, x10
00000001000014f0         add        x20, x8, #0x8
00000001000014f4         add        x8, x9, x9, lsl #1
00000001000014f8         lsl        x21, x8, #0x3
00000001000014fc         b          loc_10000150c

                     loc_100001500:
0000000100001500         add        x20, x20, #0x18                             ; CODE XREF=__ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7c019c8f90d821d9E+80, __ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7c019c8f90d821d9E+96
0000000100001504         subs       x21, x21, #0x18
0000000100001508         b.eq       loc_100001524

                     loc_10000150c:
000000010000150c         ldr        x1, [x20]                                   ; CODE XREF=__ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7c019c8f90d821d9E+60
0000000100001510         cbz        x1, loc_100001500

0000000100001514         ldur       x0, [x20, #-0x8]
0000000100001518         movz       w2, #0x1
000000010000151c         bl         ___rust_dealloc                             ; ___rust_dealloc
0000000100001520         b          loc_100001500

                     loc_100001524:
0000000100001524         ldr        x8, [x19, #0x8]                             ; CODE XREF=__ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7c019c8f90d821d9E+28, __ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7c019c8f90d821d9E+72
0000000100001528         cmp        x8, #0x0
000000010000152c         add        x8, x8, x8, lsl #1
0000000100001530         lsl        x1, x8, #0x3
0000000100001534         ccmp       x1, #0x0, #0x4
0000000100001538         b.ne       loc_10000154c

000000010000153c         ldp        x29, x30, [sp, #0x20]
0000000100001540         ldp        x20, x19, [sp, #0x10]
0000000100001544         ldp        x22, x21, [sp], #0x30
0000000100001548         ret
                        ; endp

                     loc_10000154c:
000000010000154c         ldr        x0, [x19]                                   ; CODE XREF=__ZN4core3ptr35drop_in_place$LT$std..env..Args$GT$17h7c019c8f90d821d9E+120
0000000100001550         movz       w2, #0x8
0000000100001554         ldp        x29, x30, [sp, #0x20]
0000000100001558         ldp        x20, x19, [sp, #0x10]
000000010000155c         ldp        x22, x21, [sp], #0x30
0000000100001560         b          ___rust_dealloc                             ; ___rust_dealloc
```
