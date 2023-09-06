import type { languages } from "monaco-editor"

export const monarchDefinition: languages.IMonarchLanguage = ({
    ignoreCase: true,
    keywords: [
        "add",
        "addw",
        "sub",
        "subw",
        "addi",
        "addiw",
        "slt",
        "slti",
        "sltu",
        "sltiu",
        "lui",
        "auipc",
        "and",
        "or",
        "xor",
        "andi",
        "ori",
        "xori",
        "sll",
        "sllw",
        "srl",
        "srlw",
        "sra",
        "sraw",
        "slli",
        "slliw",
        "srli",
        "srliw",
        "srai",
        "sraiw",
        "ld",
        "lw",
        "lh",
        "lb",
        "lwu",
        "lhu",
        "lbu",
        "sd",
        "sw",
        "sh",
        "sb",
        "beq",
        "bne",
        "bge",
        "bgeu",
        "blt",
        "bltu",
        "jal",
        "jalr",
        "csrrw",
        "csrrs",
        "csrrc",
        "csrrwi",
        "csrrsi",
        "csrrci",
        "mul",
        "mulh",
        "mulhsu",
        "mulhu",
        "div",
        "divu",
        "rem",
        "remu",
        "mulw",
        "divw",
        "divuw",
        "remw",
        "remuw",
        "flw",
        "fsw",
        "fmadd.s",
        "fmsub.s",
        "fnmsub.s",
        "fnmadd.s",
        "fadd.s",
        "fsub.s",
        "fmul.s",
        "fdiv.s",
        "fsqrt.s",
        "fsgnj.s",
        "fsgnjn.s",
        "fsgnjx.s",
        "fmin.s",
        "fmax.s",
        "fcvt.w.s",
        "fcvt.wu.s",
        "fmv.x.w",
        "feq.s",
        "flt.s",
        "fle.s",
        "fclass.s",
        "fcvt.s.w",
        "fcvt.s.wu",
        "fmv.w.x",
        "fcvt.l.s",
        "fcvt.lu.s",
        "fcvt.s.l",
        "fcvt.s.lu",
        "fld",
        "fsd",
        "fmadd.d",
        "fmsub.d",
        "fnmsub.d",
        "fnmadd.d",
        "fadd.d",
        "fsub.d",
        "fmul.d",
        "fdiv.d",
        "fsqrt.d",
        "fsgnj.d",
        "fsgnjn.d",
        "fsgnjx.d",
        "fmin.d",
        "fmax.d",
        "fcvt.s.d",
        "fcvt.d.s",
        "feq.d",
        "flt.d",
        "fle.d",
        "fclass.d",
        "fcvt.w.d",
        "fcvt.wu.d",
        "fcvt.d.w",
        "fcvt.d.wu",
        "fcvt.l.d",
        "fcvt.lu.d",
        "fmv.x.d",
        "fcvt.d.l",
        "fcvt.d.lu",
        "fmv.d.x",
        "vsetvli",
        "vsetivli",
        "vsetvl",
        "vle8.v",
        "vle16.v",
        "vle32.v",
        "vle64.v",
        "vse8.v",
        "vse16.v",
        "vse32.v",
        "vse64.v",
        "vlm.v",
        "vsm.v",
        "vlse8.v",
        "vlse16.v",
        "vlse32.v",
        "vlse64.v",
        "vsse8.v",
        "vsse16.v",
        "vsse32.v",
        "vsse64.v",
        "vluxei8.v",
        "vluxei16.v",
        "vluxei32.v",
        "vluxei64.v",
        "vloxei8.v",
        "vloxei16.v",
        "vloxei32.v",
        "vloxei64.v",
        "vsuxei8.v",
        "vsuxei16.v",
        "vsuxei32.v",
        "vsuxei64.v",
        "vsuxeix8.v",
        "vsuxeix16.v",
        "vsuxeix32.v",
        "vsuxeix64.v",
        "vle8ff.v",
        "vle16ff.v",
        "vle32ff.v",
        "vle64ff.v",
        "vlseg1e8.v",
        "vlseg1e16.v",
        "vlseg1e32.v",
        "vlseg1e64.v",
        "vlseg2e8.v",
        "vlseg2e16.v",
        "vlseg2e32.v",
        "vlseg2e64.v",
        "vlseg3e8.v",
        "vlseg3e16.v",
        "vlseg3e32.v",
        "vlseg3e64.v",
        "vlseg4e8.v",
        "vlseg4e16.v",
        "vlseg4e32.v",
        "vlseg4e64.v",
        "vlseg5e8.v",
        "vlseg5e16.v",
        "vlseg5e32.v",
        "vlseg5e64.v",
        "vlseg6e8.v",
        "vlseg6e16.v",
        "vlseg6e32.v",
        "vlseg6e64.v",
        "vlseg7e8.v",
        "vlseg7e16.v",
        "vlseg7e32.v",
        "vlseg7e64.v",
        "vlseg8e8.v",
        "vlseg8e16.v",
        "vlseg8e32.v",
        "vlseg8e64.v",
        "vsseg1e8.v",
        "vsseg1e16.v",
        "vsseg1e32.v",
        "vsseg1e64.v",
        "vsseg2e8.v",
        "vsseg2e16.v",
        "vsseg2e32.v",
        "vsseg2e64.v",
        "vsseg3e8.v",
        "vsseg3e16.v",
        "vsseg3e32.v",
        "vsseg3e64.v",
        "vsseg4e8.v",
        "vsseg4e16.v",
        "vsseg4e32.v",
        "vsseg4e64.v",
        "vsseg5e8.v",
        "vsseg5e16.v",
        "vsseg5e32.v",
        "vsseg5e64.v",
        "vsseg6e8.v",
        "vsseg6e16.v",
        "vsseg6e32.v",
        "vsseg6e64.v",
        "vsseg7e8.v",
        "vsseg7e16.v",
        "vsseg7e32.v",
        "vsseg7e64.v",
        "vsseg8e8.v",
        "vsseg8e16.v",
        "vsseg8e32.v",
        "vsseg8e64.v",
        "vlsseg1e8.v",
        "vlsseg1e16.v",
        "vlsseg1e32.v",
        "vlsseg1e64.v",
        "vlsseg2e8.v",
        "vlsseg2e16.v",
        "vlsseg2e32.v",
        "vlsseg2e64.v",
        "vlsseg3e8.v",
        "vlsseg3e16.v",
        "vlsseg3e32.v",
        "vlsseg3e64.v",
        "vlsseg4e8.v",
        "vlsseg4e16.v",
        "vlsseg4e32.v",
        "vlsseg4e64.v",
        "vlsseg5e8.v",
        "vlsseg5e16.v",
        "vlsseg5e32.v",
        "vlsseg5e64.v",
        "vlsseg6e8.v",
        "vlsseg6e16.v",
        "vlsseg6e32.v",
        "vlsseg6e64.v",
        "vlsseg7e8.v",
        "vlsseg7e16.v",
        "vlsseg7e32.v",
        "vlsseg7e64.v",
        "vlsseg8e8.v",
        "vlsseg8e16.v",
        "vlsseg8e32.v",
        "vlsseg8e64.v",
        "vssseg1e8.v",
        "vssseg1e16.v",
        "vssseg1e32.v",
        "vssseg1e64.v",
        "vssseg2e8.v",
        "vssseg2e16.v",
        "vssseg2e32.v",
        "vssseg2e64.v",
        "vssseg3e8.v",
        "vssseg3e16.v",
        "vssseg3e32.v",
        "vssseg3e64.v",
        "vssseg4e8.v",
        "vssseg4e16.v",
        "vssseg4e32.v",
        "vssseg4e64.v",
        "vssseg5e8.v",
        "vssseg5e16.v",
        "vssseg5e32.v",
        "vssseg5e64.v",
        "vssseg6e8.v",
        "vssseg6e16.v",
        "vssseg6e32.v",
        "vssseg6e64.v",
        "vssseg7e8.v",
        "vssseg7e16.v",
        "vssseg7e32.v",
        "vssseg7e64.v",
        "vssseg8e8.v",
        "vssseg8e16.v",
        "vssseg8e32.v",
        "vssseg8e64.v",
        "vluxseg1ei8.v",
        "vluxseg1ei16.v",
        "vluxseg1ei32.v",
        "vluxseg1ei64.v",
        "vluxseg2ei8.v",
        "vluxseg2ei16.v",
        "vluxseg2ei32.v",
        "vluxseg2ei64.v",
        "vluxseg3ei8.v",
        "vluxseg3ei16.v",
        "vluxseg3ei32.v",
        "vluxseg3ei64.v",
        "vluxseg4ei8.v",
        "vluxseg4ei16.v",
        "vluxseg4ei32.v",
        "vluxseg4ei64.v",
        "vluxseg5ei8.v",
        "vluxseg5ei16.v",
        "vluxseg5ei32.v",
        "vluxseg5ei64.v",
        "vluxseg6ei8.v",
        "vluxseg6ei16.v",
        "vluxseg6ei32.v",
        "vluxseg6ei64.v",
        "vluxseg7ei8.v",
        "vluxseg7ei16.v",
        "vluxseg7ei32.v",
        "vluxseg7ei64.v",
        "vluxseg8ei8.v",
        "vluxseg8ei16.v",
        "vluxseg8ei32.v",
        "vluxseg8ei64.v",
        "vloxseg1ei8.v",
        "vloxseg1ei16.v",
        "vloxseg1ei32.v",
        "vloxseg1ei64.v",
        "vloxseg2ei8.v",
        "vloxseg2ei16.v",
        "vloxseg2ei32.v",
        "vloxseg2ei64.v",
        "vloxseg3ei8.v",
        "vloxseg3ei16.v",
        "vloxseg3ei32.v",
        "vloxseg3ei64.v",
        "vloxseg4ei8.v",
        "vloxseg4ei16.v",
        "vloxseg4ei32.v",
        "vloxseg4ei64.v",
        "vloxseg5ei8.v",
        "vloxseg5ei16.v",
        "vloxseg5ei32.v",
        "vloxseg5ei64.v",
        "vloxseg6ei8.v",
        "vloxseg6ei16.v",
        "vloxseg6ei32.v",
        "vloxseg6ei64.v",
        "vloxseg7ei8.v",
        "vloxseg7ei16.v",
        "vloxseg7ei32.v",
        "vloxseg7ei64.v",
        "vloxseg8ei8.v",
        "vloxseg8ei16.v",
        "vloxseg8ei32.v",
        "vloxseg8ei64.v",
        "vsuxseg1ei8.v",
        "vsuxseg1ei16.v",
        "vsuxseg1ei32.v",
        "vsuxseg1ei64.v",
        "vsuxseg2ei8.v",
        "vsuxseg2ei16.v",
        "vsuxseg2ei32.v",
        "vsuxseg2ei64.v",
        "vsuxseg3ei8.v",
        "vsuxseg3ei16.v",
        "vsuxseg3ei32.v",
        "vsuxseg3ei64.v",
        "vsuxseg4ei8.v",
        "vsuxseg4ei16.v",
        "vsuxseg4ei32.v",
        "vsuxseg4ei64.v",
        "vsuxseg5ei8.v",
        "vsuxseg5ei16.v",
        "vsuxseg5ei32.v",
        "vsuxseg5ei64.v",
        "vsuxseg6ei8.v",
        "vsuxseg6ei16.v",
        "vsuxseg6ei32.v",
        "vsuxseg6ei64.v",
        "vsuxseg7ei8.v",
        "vsuxseg7ei16.v",
        "vsuxseg7ei32.v",
        "vsuxseg7ei64.v",
        "vsuxseg8ei8.v",
        "vsuxseg8ei16.v",
        "vsuxseg8ei32.v",
        "vsuxseg8ei64.v",
        "vsoxseg1ei8.v",
        "vsoxseg1ei16.v",
        "vsoxseg1ei32.v",
        "vsoxseg1ei64.v",
        "vsoxseg2ei8.v",
        "vsoxseg2ei16.v",
        "vsoxseg2ei32.v",
        "vsoxseg2ei64.v",
        "vsoxseg3ei8.v",
        "vsoxseg3ei16.v",
        "vsoxseg3ei32.v",
        "vsoxseg3ei64.v",
        "vsoxseg4ei8.v",
        "vsoxseg4ei16.v",
        "vsoxseg4ei32.v",
        "vsoxseg4ei64.v",
        "vsoxseg5ei8.v",
        "vsoxseg5ei16.v",
        "vsoxseg5ei32.v",
        "vsoxseg5ei64.v",
        "vsoxseg6ei8.v",
        "vsoxseg6ei16.v",
        "vsoxseg6ei32.v",
        "vsoxseg6ei64.v",
        "vsoxseg7ei8.v",
        "vsoxseg7ei16.v",
        "vsoxseg7ei32.v",
        "vsoxseg7ei64.v",
        "vsoxseg8ei8.v",
        "vsoxseg8ei16.v",
        "vsoxseg8ei32.v",
        "vsoxseg8ei64.v",
        "vl1re8.v",
        "vl1re16.v",
        "vl1re32.v",
        "vl1re64.v",
        "vl2re8.v",
        "vl2re16.v",
        "vl2re32.v",
        "vl2re64.v",
        "vl4re8.v",
        "vl4re16.v",
        "vl4re32.v",
        "vl4re64.v",
        "vl8re8.v",
        "vl8re16.v",
        "vl8re32.v",
        "vl8re64.v",
        "vs1r.v",
        "vs2r.v",
        "vs4r.v",
        "vs8r.v",
        "vadd.vv",
        "vadd.vx",
        "vadd.vi",
        "vsub.vv",
        "vsub.vx",
        "vrsub.vx",
        "vrsub.vi",
        "vminu.vv",
        "vminu.vx",
        "vmin.vv",
        "vmin.vx",
        "vmaxu.vv",
        "vmaxu.vx",
        "vmax.vv",
        "vmax.vx",
        "vand.vv",
        "vand.vx",
        "vand.vi",
        "vor.vv",
        "vor.vx",
        "vor.vi",
        "vxor.vv",
        "vxor.vx",
        "vxor.vi",
        "vrgather.vv",
        "vrgather.vx",
        "vrgather.vi",
        "vrgatherei16.v",
        "vslideup.vx",
        "vslideup.vi",
        "vslidedown.vx",
        "vslidedown.vi",
        "vadc.vvm",
        "vadc.vxm",
        "vadc.vim",
        "vmadc.vvm",
        "vmadc.vxm",
        "vmadc.vim",
        "vmadc.vv",
        "vmadc.vx",
        "vmadc.vi",
        "vsbc.vvm",
        "vsbc.vxm",
        "vmsbc.vv",
        "vmsbc.vx",
        "vmerge.vvm",
        "vmerge.vxm",
        "vmerge.vim",
        "vmv.v.v",
        "vmv.v.x",
        "vmv.v.i",
        "vmseq.vv",
        "vmseq.vx",
        "vmseq.vi",
        "vmsne.vv",
        "vmsne.vx",
        "vmsne.vi",
        "vmsltu.vv",
        "vmsltu.vx",
        "vmslt.vv",
        "vmslt.vx",
        "vmsleu.vv",
        "vmsleu.vx",
        "vmsleu.vi",
        "vmsle.vv",
        "vmsle.vx",
        "vmsle.vi",
        "vmsgtu.vx",
        "vmsgtu.vi",
        "vmsgt.vx",
        "vmsgt.vi",
        "vsaddu.vv",
        "vsaddu.vx",
        "vsaddu.vi",
        "vsadd.vv",
        "vsadd.vx",
        "vsadd.vi",
        "vssubu.vv",
        "vssubu.vx",
        "vssub.vv",
        "vssub.vx",
        "vsll.vv",
        "vsll.vx",
        "vsll.vi",
        "vsmul.vv",
        "vsmul.vx",
        "vmv1r.v",
        "vmv2r.v",
        "vmv4r.v",
        "vmv8r.v",
        "vsrl.vv",
        "vsrl.vx",
        "vsrl.vi",
        "vsra.vv",
        "vsra.vx",
        "vsra.vi",
        "vssrl.vv",
        "vssrl.vx",
        "vssrl.vi",
        "vssra.vv",
        "vssra.vx",
        "vssra.vi",
        "vnsrl.wv",
        "vnsrl.wx",
        "vnsrl.wi",
        "vnsra.wv",
        "vnsra.wx",
        "vnsra.wi",
        "vnclipu.wv",
        "vnclipu.wx",
        "vnclipu.wi",
        "vnclip.wv",
        "vnclip.wx",
        "vnclip.wi",
        "vwredsumu.vs",
        "vwredsum.vs",
        "vredsum.vs",
        "vredand.vs",
        "vredor.vs",
        "vredxor.vs",
        "vredminu.vs",
        "vredmin.vs",
        "vredmaxu.vs",
        "vredmax.vs",
        "vaaddu.vv",
        "vaaddu.vx",
        "vaadd.vv",
        "vaadd.vx",
        "vasubu.vv",
        "vasubu.vx",
        "vasub.vv",
        "vasub.vx",
        "vslide1up.vx",
        "vslide1down.vx",
        "vmv.x.s",
        "vcpop.m",
        "vfirst.m",
        "vmv.s.x",
        "vsext.vf2",
        "vsext.vf4",
        "vsext.vf8",
        "vzext.vf2",
        "vzext.vf4",
        "vzext.vf8",
        "vmsbf.m",
        "vmsof.m",
        "vmsif.m",
        "viota.m",
        "vid.v",
        "vcompress.vm",
        "vmandn.mm",
        "vmand.mm",
        "vmor.mm",
        "vmxor.mm",
        "vmorn.mm",
        "vmnand.mm",
        "vmnor.mm",
        "vmxnor.mm",
        "vdivu.vv",
        "vdivu.vx",
        "vdiv.vv",
        "vdiv.vx",
        "vremu.vv",
        "vremu.vx",
        "vrem.vv",
        "vrem.vx",
        "vmulhu.vv",
        "vmulhu.vx",
        "vmul.vv",
        "vmul.vx",
        "vmulhsu.vv",
        "vmulhsu.vx",
        "vmulh.vv",
        "vmulh.vx",
        "vmadd.vv",
        "vmadd.vx",
        "vnmsub.vv",
        "vnmsub.vx",
        "vmacc.vv",
        "vmacc.vx",
        "vnmsac.vv",
        "vnmsac.vx",
        "vwaddu.vv",
        "vwaddu.vx",
        "vwadd.vv",
        "vwadd.vx",
        "vwsubu.vv",
        "vwsubu.vx",
        "vwsub.vv",
        "vwsub.vx",
        "vwaddu.wv",
        "vwaddu.wx",
        "vwadd.wv",
        "vwadd.wx",
        "vwsubu.wv",
        "vwsubu.wx",
        "vwsub.wv",
        "vwsub.wx",
        "vwmulu.vv",
        "vwmulu.vx",
        "vwmulsu.vv",
        "vwmulsu.vx",
        "vwmul.vv",
        "vwmul.vx",
        "vwmaccu.vv",
        "vwmaccu.vx",
        "vwmacc.vv",
        "vwmacc.vx",
        "vwmaccus.vx",
        "vwmaccsu.vv",
        "vwmaccsu.vx",
        "vfadd.vv",
        "vfadd.vf",
        "vfredusum.vs",
        "vfsub.vv",
        "vfsub.vf",
        "vfredosum.vs",
        "vfmin.vv",
        "vfmin.vf",
        "vfredmin.vs",
        "vfmax.vv",
        "vfmax.vf",
        "vfredmax.vs",
        "vfsgnj.vv",
        "vfsgnj.vf",
        "vfsgnjn.vv",
        "vfsgnjn.vf",
        "vfsgnjx.vv",
        "vfsgnjx.vf",
        "vfslide1up.vf",
        "vfslide1down.vf",
        "vfmv.f.s",
        "vfmv.s.f",
        "vfcvt.xu.f.v",
        "vfcvt.x.f.v",
        "vfcvt.f.xu.v",
        "vfcvt.f.x.v",
        "vfcvt.rtz.xu.f.v",
        "vfcvt.rtz.x.f.v",
        "vfwcvt.xu.f.v",
        "vfwcvt.x.f.v",
        "vfwcvt.f.xu.v",
        "vfwcvt.f.x.v",
        "vfwcvt.f.f.v",
        "vfwcvt.rtz.xu.f.v",
        "vfwcvt.rtz.x.f.v",
        "vfncvt.xu.f.w",
        "vfncvt.x.f.w",
        "vfncvt.f.xu.w",
        "vfncvt.f.x.w",
        "vfncvt.f.f.w",
        "vfncvt.rod.f.f.w",
        "vfncvt.rtz.xu.f.w",
        "vfncvt.rtz.x.f.w",
        "vfsqrt.v",
        "vfrsqrt7.v",
        "vfrec7.v",
        "vfclass.v",
        "vfmerge.vfm",
        "vfmv.v.f",
        "vmfeq.vv",
        "vmfeq.vf",
        "vmfle.vv",
        "vmfle.vf",
        "vmflt.vv",
        "vmflt.vf",
        "vmfne.vv",
        "vmfne.vf",
        "vmfgt.vf",
        "vmfge.vf",
        "vfdiv.vv",
        "vfdiv.vf",
        "vfrdiv.vf",
        "vfmul.vv",
        "vfmul.vf",
        "vfrsub.vf",
        "vfmadd.vv",
        "vfmadd.vf",
        "vfnmadd.vv",
        "vfnmadd.vf",
        "vfmsub.vv",
        "vfmsub.vf",
        "vfnmsub.vv",
        "vfnmsub.vf",
        "vfmacc.vv",
        "vfmacc.vf",
        "vfnmacc.vv",
        "vfnmacc.vf",
        "vfmsac.vv",
        "vfmsac.vf",
        "vfnmsac.vv",
        "vfnmsac.vf",
        "vfwadd.vv",
        "vfwadd.vf",
        "vfwredusum.vs",
        "vfwsub.vv",
        "vfwsub.vf",
        "vfwredosum.vs",
        "vfwadd.wv",
        "vfwadd.wf",
        "vfwsub.wv",
        "vfwsub.wf",
        "vfwmul.vv",
        "vfwmul.vf",
        "vfwmacc.vv",
        "vfwmacc.vf",
        "vfwnmacc.vv",
        "vfwnmacc.vf",
        "vfwmsac.vv",
        "vfwmsac.vf",
        "vfwnmsac.vv",
        "vfwnmsac.vf",
        "nop",
        "li",
        "mv",
        "not",
        "neg",
        "negw",
        "sext.b",
        "sext.h",
        "sext.w",
        "zext.b",
        "zext.h",
        "zext.w",
        "seqz",
        "snez",
        "sltz",
        "sgtz",
        "fmv.s",
        "fabs.s",
        "fneg.s",
        "fmv.d",
        "fabs.d",
        "fneg.d",
        "beqz",
        "bnez",
        "blez",
        "bgez",
        "bltz",
        "bgtz",
        "bgt",
        "ble",
        "bgtu",
        "bleu",
        "j",
        "jr",
        "ret",
        "call",
        "tail",
        "rdinstret",
        "rdinstreth",
        "rdcycle",
        "rdcycleh",
        "csrr",
        "csrw",
        "csrs",
        "csrc",
        "frcsr",
        "fscsr",
        "frrm",
        "fsrm",
        "fsrmi",
        "frflags",
        "fsflags",
        "fsflagsi"
    ],

    "registers": [
        "x0", "zero",
        "x1", "ra",
        "x2", "sp",
        "x3", "gp",
        "x4", "tp",
        "x5", "t0",
        "x6", "t1",
        "x7", "t2",
        "x8", "s0", "fp",
        "x9", "s1",
        "x10", "a0",
        "x11", "a1",
        "x12", "a2",
        "x13", "a3",
        "x14", "a4",
        "x15", "a5",
        "x16", "a6",
        "x17", "a7",
        "x18", "s2",
        "x19", "s3",
        "x20", "s4",
        "x21", "s5",
        "x22", "s6",
        "x23", "s7",
        "x24", "s8",
        "x25", "s9",
        "x26", "s10",
        "x27", "s11",
        "x28", "t3",
        "x29", "t4",
        "x30", "t5",
        "x31", "t6",
        "f0", "ft0",
        "f1", "ft1",
        "f2", "ft2",
        "f3", "ft3",
        "f4", "ft4",
        "f5", "ft5",
        "f6", "ft6",
        "f7", "ft7",
        "f8", "fs0",
        "f9", "fs1",
        "f10", "fa0",
        "f11", "fa1",
        "f12", "fa2",
        "f13", "fa3",
        "f14", "fa4",
        "f15", "fa5",
        "f16", "fa6",
        "f17", "fa7",
        "f18", "fs2",
        "f19", "fs3",
        "f20", "fs4",
        "f21", "fs5",
        "f22", "fs6",
        "f23", "fs7",
        "f24", "fs8",
        "f25", "fs9",
        "f26", "fs10",
        "f27", "fs11",
        "f28", "ft8",
        "f29", "ft9",
        "f30", "ft10",
        "f31", "ft11",
        "instret",
        "instreth",
        "cycle",
        "cycleh",
        "time",
        "timeh",
        "marchid",
        "fcsr",
        "fflags",
        "frm",
        "mstatus",
        "vsstatus",
        "vtype",
        "vl",
        "vlenb",
        "vstart",
        "vxrm",
        "vxsat",
        "vcsr",
        "v0",
        "v1",
        "v2",
        "v3",
        "v4",
        "v5",
        "v6",
        "v7",
        "v8",
        "v9",
        "v10",
        "v11",
        "v12",
        "v13",
        "v14",
        "v15",
        "v16",
        "v17",
        "v18",
        "v19",
        "v20",
        "v21",
        "v22",
        "v23",
        "v24",
        "v25",
        "v26",
        "v27",
        "v28",
        "v29",
        "v30",
        "v31",
        "v0.t"
    ],

    vectorOperands: [
        "e8", "e16", "e32", "e64",
        "mf8", "mf4", "mf2", "m1", "m2", "m4", "m8",
        "ma", "mu", "ta", "tu"
    ],

    tokenizer: {
        root: [
            [/[ \t\r\n]+/, 'white'],
        
            [/#.*$/, 'comment'],
        
            [/\.[a-zA-Z_]\w*/, 'keyword'],
        
            [/[a-zA-Z_$][\w$\.]*/, { cases: { '@keywords': 'keyword',
                                            '@registers': 'variable.name',
                                            '@vectorOperands': 'regexp',
                                            '@default': 'identifier'} }],
        
            [/0[xX][0-9a-fA-F]+/, 'number.hex'],
            [/0[bB][01]+/, 'number.binary'],
            [/0[oO][0-7]+/, 'number.octal'],
            [/\d+/, 'number'],
        
            [/^[a-zA-Z_]\w*:/, 'namespace'],
            [/^[^\s]+:/, 'namespace'],
        ]
    }
});