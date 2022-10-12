#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]

static mut processor: *mut ProcessorState;
    fn VirtualMemoryReadBlock(
        vma: isize,
        object: *mut LispObj,
        count: u32,
    ) -> u32;


pub type Tag = u8;

#[derive(Copy, Clone)]
#[repr(C)]
pub union LispObj {
    pub parts: LispObjTagData,
    pub whole: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LispObjTagData {
    pub tag: ui32,
    pub data: LispObjData,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union LispObjData {
    pub u: ui32,
    pub s: i32,
    pub f: libc::c_float,
}
pub type PC = LispObj;


pub const DispatchInstructionCacheLookup: _IvoryDispatch = 722;
pub const TypeOddPC: _IvoryType = 39;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct DecoderPair {
    pub dispatch: u32,
    pub decode: Option::<fn() -> ()>,
}

pub const DispatchHack: _IvoryDispatch = 657;
pub const DispatchCatchOpen: _IvoryDispatch = 656;
pub const DispatchLoopIncrementTosLessThan: _IvoryDispatch = 655;
pub const DispatchIllegalInstruction: _IvoryDispatch = 723;
pub const DispatchPTagDpb: _IvoryDispatch = 654;
pub const DispatchPDpb: _IvoryDispatch = 653;
pub const DispatchCharDpb: _IvoryDispatch = 652;
pub const DispatchDpb: _IvoryDispatch = 651;
pub const DispatchStackBltAddressImmediate: _IvoryDispatch = 649;
pub const DispatchStackBltAddressPop: _IvoryDispatch = 650;
pub const DispatchFastAset1Pop: _IvoryDispatch = 645;
pub const DispatchFastAref1Pop: _IvoryDispatch = 640;
pub const DispatchMergeCdrNoPopPop: _IvoryDispatch = 635;
pub const DispatchMovemPop: _IvoryDispatch = 630;
pub const DispatchPopPop: _IvoryDispatch = 625;
pub const DispatchUnsignedLesspNoPopPop: _IvoryDispatch = 620;
pub const DispatchUnsignedLesspPop: _IvoryDispatch = 615;
pub const DispatchSetTagPop: _IvoryDispatch = 610;
pub const DispatchInstanceLocPop: _IvoryDispatch = 605;
pub const DispatchInstanceSetPop: _IvoryDispatch = 600;
pub const DispatchInstanceRefPop: _IvoryDispatch = 595;
pub const DispatchAlocLeaderPop: _IvoryDispatch = 586;
pub const DispatchArrayLeaderPop: _IvoryDispatch = 581;
pub const DispatchAllocateStructureBlockPop: _IvoryDispatch = 576;
pub const DispatchStoreArrayLeaderPop: _IvoryDispatch = 571;
pub const DispatchAloc1Pop: _IvoryDispatch = 566;
pub const DispatchAref1Pop: _IvoryDispatch = 561;
pub const DispatchAllocateListBlockPop: _IvoryDispatch = 556;
pub const DispatchAset1Pop: _IvoryDispatch = 551;
pub const DispatchDivideBignumStepPop: _IvoryDispatch = 546;
pub const DispatchMultiplyBignumStepPop: _IvoryDispatch = 541;
pub const DispatchSubBignumStepPop: _IvoryDispatch = 536;
pub const DispatchAddBignumStepPop: _IvoryDispatch = 531;
pub const Dispatch32BitDifferencePop: _IvoryDispatch = 526;
pub const Dispatch32BitPlusPop: _IvoryDispatch = 521;
pub const DispatchSubPop: _IvoryDispatch = 516;
pub const DispatchAddPop: _IvoryDispatch = 511;
pub const DispatchLogtestNoPopPop: _IvoryDispatch = 506;
pub const DispatchEqNoPopPop: _IvoryDispatch = 501;
pub const DispatchLogtestPop: _IvoryDispatch = 496;
pub const DispatchEqPop: _IvoryDispatch = 491;
pub const DispatchEqlNoPopPop: _IvoryDispatch = 486;
pub const DispatchGreaterpNoPopPop: _IvoryDispatch = 481;
pub const DispatchLesspNoPopPop: _IvoryDispatch = 476;
pub const DispatchEqualNumberNoPopPop: _IvoryDispatch = 471;
pub const DispatchEqlPop: _IvoryDispatch = 466;
pub const DispatchGreaterpPop: _IvoryDispatch = 461;
pub const DispatchLesspPop: _IvoryDispatch = 456;
pub const DispatchEqualNumberPop: _IvoryDispatch = 451;
pub const DispatchMovemLexicalVarPop: _IvoryDispatch = 446;
pub const DispatchPopLexicalVarPop: _IvoryDispatch = 441;
pub const DispatchUnifyPop: _IvoryDispatch = 436;
pub const DispatchBindLocativeToValuePop: _IvoryDispatch = 431;
pub const DispatchPStoreContentsPop: _IvoryDispatch = 426;
pub const DispatchMemoryWritePop: _IvoryDispatch = 421;
pub const DispatchStoreConditionalPop: _IvoryDispatch = 416;
pub const DispatchAshPop: _IvoryDispatch = 411;
pub const DispatchPointerDifferencePop: _IvoryDispatch = 406;
pub const DispatchPointerPlusPop: _IvoryDispatch = 401;
pub const DispatchAssocPop: _IvoryDispatch = 396;
pub const DispatchMemberPop: _IvoryDispatch = 391;
pub const DispatchRgetfPop: _IvoryDispatch = 386;
pub const DispatchStackBltPop: _IvoryDispatch = 381;
pub const DispatchLshcBignumStepPop: _IvoryDispatch = 376;
pub const DispatchMultiplyDoublePop: _IvoryDispatch = 371;
pub const DispatchLshPop: _IvoryDispatch = 366;
pub const DispatchRotPop: _IvoryDispatch = 361;
pub const DispatchLogiorPop: _IvoryDispatch = 356;
pub const DispatchLogxorPop: _IvoryDispatch = 351;
pub const DispatchLogandPop: _IvoryDispatch = 346;
pub const DispatchAluPop: _IvoryDispatch = 341;
pub const DispatchMaxPop: _IvoryDispatch = 336;
pub const DispatchMinPop: _IvoryDispatch = 331;
pub const DispatchRationalQuotientPop: _IvoryDispatch = 326;
pub const DispatchRoundPop: _IvoryDispatch = 321;
pub const DispatchTruncatePop: _IvoryDispatch = 316;
pub const DispatchFloorPop: _IvoryDispatch = 311;
pub const DispatchCeilingPop: _IvoryDispatch = 306;
pub const DispatchQuotientPop: _IvoryDispatch = 301;
pub const DispatchMultiplyPop: _IvoryDispatch = 296;
pub const DispatchRplacdPop: _IvoryDispatch = 291;
pub const DispatchRplacaPop: _IvoryDispatch = 286;
pub const DispatchBlock3ReadAluPop: _IvoryDispatch = 273;
pub const DispatchBlock2ReadAluPop: _IvoryDispatch = 268;
pub const DispatchBlock1ReadAluPop: _IvoryDispatch = 263;
pub const DispatchBlock0ReadAluPop: _IvoryDispatch = 258;
pub const DispatchSetSpToAddressSaveTosPop: _IvoryDispatch = 249;
pub const DispatchSetSpToAddressPop: _IvoryDispatch = 244;
pub const DispatchPushAddressPop: _IvoryDispatch = 239;
pub const DispatchSetCdrCode2Pop: _IvoryDispatch = 234;
pub const DispatchSetCdrCode1Pop: _IvoryDispatch = 229;
pub const DispatchPointerIncrementPop: _IvoryDispatch = 224;
pub const DispatchDecrementPop: _IvoryDispatch = 219;
pub const DispatchIncrementPop: _IvoryDispatch = 214;
pub const DispatchSetToCdrPushCarPop: _IvoryDispatch = 209;
pub const DispatchSetToCdrPop: _IvoryDispatch = 204;
pub const DispatchSetToCarPop: _IvoryDispatch = 199;
pub const DispatchUnaryMinusPop: _IvoryDispatch = 173;
pub const DispatchUnbindNPop: _IvoryDispatch = 164;
pub const DispatchReturnKludgePop: _IvoryDispatch = 161;
pub const DispatchReturnMultiplePop: _IvoryDispatch = 156;
pub const DispatchPushLocalLogicVariablesPop: _IvoryDispatch = 151;
pub const DispatchPushAddressSpRelativePop: _IvoryDispatch = 146;
pub const DispatchPushPop: _IvoryDispatch = 140;
pub const DispatchPluspPop: _IvoryDispatch = 109;
pub const DispatchMinuspPop: _IvoryDispatch = 104;
pub const DispatchZeropPop: _IvoryDispatch = 99;
pub const DispatchBlock3WritePop: _IvoryDispatch = 94;
pub const DispatchBlock2WritePop: _IvoryDispatch = 89;
pub const DispatchBlock1WritePop: _IvoryDispatch = 84;
pub const DispatchBlock0WritePop: _IvoryDispatch = 79;
pub const DispatchPushLexicalVarPop: _IvoryDispatch = 74;
pub const DispatchProcBreakpointPop: _IvoryDispatch = 69;
pub const DispatchLogicTailTestPop: _IvoryDispatch = 64;
pub const DispatchDereferencePop: _IvoryDispatch = 59;
pub const DispatchTagPop: _IvoryDispatch = 54;
pub const DispatchJumpPop: _IvoryDispatch = 49;
pub const DispatchStartCallPop: _IvoryDispatch = 44;
pub const DispatchEphemeralpPop: _IvoryDispatch = 39;
pub const DispatchRestoreBindingStackPop: _IvoryDispatch = 34;
pub const DispatchBindLocativePop: _IvoryDispatch = 29;
pub const DispatchSetupForce1dArrayPop: _IvoryDispatch = 24;
pub const DispatchSetup1dArrayPop: _IvoryDispatch = 19;
pub const DispatchEndpPop: _IvoryDispatch = 14;
pub const DispatchCdrPop: _IvoryDispatch = 9;
pub const DispatchCarPop: _IvoryDispatch = 4;
pub const DispatchStackBltAddressSP: _IvoryDispatch = 648;
pub const DispatchStackBltAddressLP: _IvoryDispatch = 647;
pub const DispatchStackBltAddressFP: _IvoryDispatch = 646;
pub const DispatchFastAset1Immediate: _IvoryDispatch = 644;
pub const DispatchFastAset1SP: _IvoryDispatch = 643;
pub const DispatchFastAset1LP: _IvoryDispatch = 642;
pub const DispatchFastAset1FP: _IvoryDispatch = 641;
pub const DispatchFastAref1Immediate: _IvoryDispatch = 639;
pub const DispatchFastAref1SP: _IvoryDispatch = 638;
pub const DispatchFastAref1LP: _IvoryDispatch = 637;
pub const DispatchFastAref1FP: _IvoryDispatch = 636;
pub const DispatchMergeCdrNoPopImmediate: _IvoryDispatch = 634;
pub const DispatchMergeCdrNoPopSP: _IvoryDispatch = 633;
pub const DispatchMergeCdrNoPopLP: _IvoryDispatch = 632;
pub const DispatchMergeCdrNoPopFP: _IvoryDispatch = 631;
pub const DispatchMovemImmediate: _IvoryDispatch = 629;
pub const DispatchMovemSP: _IvoryDispatch = 628;
pub const DispatchMovemLP: _IvoryDispatch = 627;
pub const DispatchMovemFP: _IvoryDispatch = 626;
pub const DispatchPopImmediate: _IvoryDispatch = 624;
pub const DispatchPopSP: _IvoryDispatch = 623;
pub const DispatchPopLP: _IvoryDispatch = 622;
pub const DispatchPopFP: _IvoryDispatch = 621;
pub const DispatchUnsignedLesspNoPopImmediate: _IvoryDispatch = 619;
pub const DispatchUnsignedLesspNoPopSP: _IvoryDispatch = 618;
pub const DispatchUnsignedLesspNoPopLP: _IvoryDispatch = 617;
pub const DispatchUnsignedLesspNoPopFP: _IvoryDispatch = 616;
pub const DispatchUnsignedLesspImmediate: _IvoryDispatch = 614;
pub const DispatchUnsignedLesspSP: _IvoryDispatch = 613;
pub const DispatchUnsignedLesspLP: _IvoryDispatch = 612;
pub const DispatchUnsignedLesspFP: _IvoryDispatch = 611;
pub const DispatchSetTagImmediate: _IvoryDispatch = 609;
pub const DispatchSetTagSP: _IvoryDispatch = 608;
pub const DispatchSetTagLP: _IvoryDispatch = 607;
pub const DispatchSetTagFP: _IvoryDispatch = 606;
pub const DispatchInstanceLocImmediate: _IvoryDispatch = 604;
pub const DispatchInstanceLocSP: _IvoryDispatch = 603;
pub const DispatchInstanceLocLP: _IvoryDispatch = 602;
pub const DispatchInstanceLocFP: _IvoryDispatch = 601;
pub const DispatchInstanceSetImmediate: _IvoryDispatch = 599;
pub const DispatchInstanceSetSP: _IvoryDispatch = 598;
pub const DispatchInstanceSetLP: _IvoryDispatch = 597;
pub const DispatchInstanceSetFP: _IvoryDispatch = 596;
pub const DispatchInstanceRefImmediate: _IvoryDispatch = 594;
pub const DispatchInstanceRefSP: _IvoryDispatch = 593;
pub const DispatchInstanceRefLP: _IvoryDispatch = 592;
pub const DispatchInstanceRefFP: _IvoryDispatch = 591;
pub const DispatchMovemInstanceVariableOrdered: _IvoryDispatch = 590;
pub const DispatchPopInstanceVariableOrdered: _IvoryDispatch = 589;
pub const DispatchMovemInstanceVariable: _IvoryDispatch = 588;
pub const DispatchPopInstanceVariable: _IvoryDispatch = 587;
pub const DispatchAlocLeaderImmediate: _IvoryDispatch = 585;
pub const DispatchAlocLeaderSP: _IvoryDispatch = 584;
pub const DispatchAlocLeaderLP: _IvoryDispatch = 583;
pub const DispatchAlocLeaderFP: _IvoryDispatch = 582;
pub const DispatchArrayLeaderImmediate: _IvoryDispatch = 580;
pub const DispatchArrayLeaderSP: _IvoryDispatch = 579;
pub const DispatchArrayLeaderLP: _IvoryDispatch = 578;
pub const DispatchArrayLeaderFP: _IvoryDispatch = 577;
pub const DispatchAllocateStructureBlockImmediate: _IvoryDispatch = 575;
pub const DispatchAllocateStructureBlockSP: _IvoryDispatch = 574;
pub const DispatchAllocateStructureBlockLP: _IvoryDispatch = 573;
pub const DispatchAllocateStructureBlockFP: _IvoryDispatch = 572;
pub const DispatchStoreArrayLeaderImmediate: _IvoryDispatch = 570;
pub const DispatchStoreArrayLeaderSP: _IvoryDispatch = 569;
pub const DispatchStoreArrayLeaderLP: _IvoryDispatch = 568;
pub const DispatchStoreArrayLeaderFP: _IvoryDispatch = 567;
pub const DispatchAloc1Immediate: _IvoryDispatch = 565;
pub const DispatchAloc1SP: _IvoryDispatch = 564;
pub const DispatchAloc1LP: _IvoryDispatch = 563;
pub const DispatchAloc1FP: _IvoryDispatch = 562;
pub const DispatchAref1Immediate: _IvoryDispatch = 560;
pub const DispatchAref1SP: _IvoryDispatch = 559;
pub const DispatchAref1LP: _IvoryDispatch = 558;
pub const DispatchAref1FP: _IvoryDispatch = 557;
pub const DispatchAllocateListBlockImmediate: _IvoryDispatch = 555;
pub const DispatchAllocateListBlockSP: _IvoryDispatch = 554;
pub const DispatchAllocateListBlockLP: _IvoryDispatch = 553;
pub const DispatchAllocateListBlockFP: _IvoryDispatch = 552;
pub const DispatchAset1Immediate: _IvoryDispatch = 550;
pub const DispatchAset1SP: _IvoryDispatch = 549;
pub const DispatchAset1LP: _IvoryDispatch = 548;
pub const DispatchAset1FP: _IvoryDispatch = 547;
pub const DispatchDivideBignumStepImmediate: _IvoryDispatch = 545;
pub const DispatchDivideBignumStepSP: _IvoryDispatch = 544;
pub const DispatchDivideBignumStepLP: _IvoryDispatch = 543;
pub const DispatchDivideBignumStepFP: _IvoryDispatch = 542;
pub const DispatchMultiplyBignumStepImmediate: _IvoryDispatch = 540;
pub const DispatchMultiplyBignumStepSP: _IvoryDispatch = 539;
pub const DispatchMultiplyBignumStepLP: _IvoryDispatch = 538;
pub const DispatchMultiplyBignumStepFP: _IvoryDispatch = 537;
pub const DispatchSubBignumStepImmediate: _IvoryDispatch = 535;
pub const DispatchSubBignumStepSP: _IvoryDispatch = 534;
pub const DispatchSubBignumStepLP: _IvoryDispatch = 533;
pub const DispatchSubBignumStepFP: _IvoryDispatch = 532;
pub const DispatchAddBignumStepImmediate: _IvoryDispatch = 530;
pub const DispatchAddBignumStepSP: _IvoryDispatch = 529;
pub const DispatchAddBignumStepLP: _IvoryDispatch = 528;
pub const DispatchAddBignumStepFP: _IvoryDispatch = 527;
pub const Dispatch32BitDifferenceImmediate: _IvoryDispatch = 525;
pub const Dispatch32BitDifferenceSP: _IvoryDispatch = 524;
pub const Dispatch32BitDifferenceLP: _IvoryDispatch = 523;
pub const Dispatch32BitDifferenceFP: _IvoryDispatch = 522;
pub const Dispatch32BitPlusImmediate: _IvoryDispatch = 520;
pub const Dispatch32BitPlusSP: _IvoryDispatch = 519;
pub const Dispatch32BitPlusLP: _IvoryDispatch = 518;
pub const Dispatch32BitPlusFP: _IvoryDispatch = 517;
pub const DispatchSubImmediate: _IvoryDispatch = 515;
pub const DispatchSubSP: _IvoryDispatch = 514;
pub const DispatchSubLP: _IvoryDispatch = 513;
pub const DispatchSubFP: _IvoryDispatch = 512;
pub const DispatchAddImmediate: _IvoryDispatch = 510;
pub const DispatchAddSP: _IvoryDispatch = 509;
pub const DispatchAddLP: _IvoryDispatch = 508;
pub const DispatchAddFP: _IvoryDispatch = 507;
pub const DispatchLogtestNoPopImmediate: _IvoryDispatch = 505;
pub const DispatchLogtestNoPopSP: _IvoryDispatch = 504;
pub const DispatchLogtestNoPopLP: _IvoryDispatch = 503;
pub const DispatchLogtestNoPopFP: _IvoryDispatch = 502;
pub const DispatchEqNoPopImmediate: _IvoryDispatch = 500;
pub const DispatchEqNoPopSP: _IvoryDispatch = 499;
pub const DispatchEqNoPopLP: _IvoryDispatch = 498;
pub const DispatchEqNoPopFP: _IvoryDispatch = 497;
pub const DispatchLogtestImmediate: _IvoryDispatch = 495;
pub const DispatchLogtestSP: _IvoryDispatch = 494;
pub const DispatchLogtestLP: _IvoryDispatch = 493;
pub const DispatchLogtestFP: _IvoryDispatch = 492;
pub const DispatchEqImmediate: _IvoryDispatch = 490;
pub const DispatchEqSP: _IvoryDispatch = 489;
pub const DispatchEqLP: _IvoryDispatch = 488;
pub const DispatchEqFP: _IvoryDispatch = 487;
pub const DispatchEqlNoPopImmediate: _IvoryDispatch = 485;
pub const DispatchEqlNoPopSP: _IvoryDispatch = 484;
pub const DispatchEqlNoPopLP: _IvoryDispatch = 483;
pub const DispatchEqlNoPopFP: _IvoryDispatch = 482;
pub const DispatchGreaterpNoPopImmediate: _IvoryDispatch = 480;
pub const DispatchGreaterpNoPopSP: _IvoryDispatch = 479;
pub const DispatchGreaterpNoPopLP: _IvoryDispatch = 478;
pub const DispatchGreaterpNoPopFP: _IvoryDispatch = 477;
pub const DispatchLesspNoPopImmediate: _IvoryDispatch = 475;
pub const DispatchLesspNoPopSP: _IvoryDispatch = 474;
pub const DispatchLesspNoPopLP: _IvoryDispatch = 473;
pub const DispatchLesspNoPopFP: _IvoryDispatch = 472;
pub const DispatchEqualNumberNoPopImmediate: _IvoryDispatch = 470;
pub const DispatchEqualNumberNoPopSP: _IvoryDispatch = 469;
pub const DispatchEqualNumberNoPopLP: _IvoryDispatch = 468;
pub const DispatchEqualNumberNoPopFP: _IvoryDispatch = 467;
pub const DispatchEqlImmediate: _IvoryDispatch = 465;
pub const DispatchEqlSP: _IvoryDispatch = 464;
pub const DispatchEqlLP: _IvoryDispatch = 463;
pub const DispatchEqlFP: _IvoryDispatch = 462;
pub const DispatchGreaterpImmediate: _IvoryDispatch = 460;
pub const DispatchGreaterpSP: _IvoryDispatch = 459;
pub const DispatchGreaterpLP: _IvoryDispatch = 458;
pub const DispatchGreaterpFP: _IvoryDispatch = 457;
pub const DispatchLesspImmediate: _IvoryDispatch = 455;
pub const DispatchLesspSP: _IvoryDispatch = 454;
pub const DispatchLesspLP: _IvoryDispatch = 453;
pub const DispatchLesspFP: _IvoryDispatch = 452;
pub const DispatchEqualNumberImmediate: _IvoryDispatch = 450;
pub const DispatchEqualNumberSP: _IvoryDispatch = 449;
pub const DispatchEqualNumberLP: _IvoryDispatch = 448;
pub const DispatchEqualNumberFP: _IvoryDispatch = 447;
pub const DispatchMovemLexicalVarImmediate: _IvoryDispatch = 445;
pub const DispatchMovemLexicalVarSP: _IvoryDispatch = 444;
pub const DispatchMovemLexicalVarLP: _IvoryDispatch = 443;
pub const DispatchMovemLexicalVarFP: _IvoryDispatch = 442;
pub const DispatchPopLexicalVarImmediate: _IvoryDispatch = 440;
pub const DispatchPopLexicalVarSP: _IvoryDispatch = 439;
pub const DispatchPopLexicalVarLP: _IvoryDispatch = 438;
pub const DispatchPopLexicalVarFP: _IvoryDispatch = 437;
pub const DispatchUnifyImmediate: _IvoryDispatch = 435;
pub const DispatchUnifySP: _IvoryDispatch = 434;
pub const DispatchUnifyLP: _IvoryDispatch = 433;
pub const DispatchUnifyFP: _IvoryDispatch = 432;
pub const DispatchBindLocativeToValueImmediate: _IvoryDispatch = 430;
pub const DispatchBindLocativeToValueSP: _IvoryDispatch = 429;
pub const DispatchBindLocativeToValueLP: _IvoryDispatch = 428;
pub const DispatchBindLocativeToValueFP: _IvoryDispatch = 427;
pub const DispatchPStoreContentsImmediate: _IvoryDispatch = 425;
pub const DispatchPStoreContentsSP: _IvoryDispatch = 424;
pub const DispatchPStoreContentsLP: _IvoryDispatch = 423;
pub const DispatchPStoreContentsFP: _IvoryDispatch = 422;
pub const DispatchMemoryWriteImmediate: _IvoryDispatch = 420;
pub const DispatchMemoryWriteSP: _IvoryDispatch = 419;
pub const DispatchMemoryWriteLP: _IvoryDispatch = 418;
pub const DispatchMemoryWriteFP: _IvoryDispatch = 417;
pub const DispatchStoreConditionalImmediate: _IvoryDispatch = 415;
pub const DispatchStoreConditionalSP: _IvoryDispatch = 414;
pub const DispatchStoreConditionalLP: _IvoryDispatch = 413;
pub const DispatchStoreConditionalFP: _IvoryDispatch = 412;
pub const DispatchAshImmediate: _IvoryDispatch = 410;
pub const DispatchAshSP: _IvoryDispatch = 409;
pub const DispatchAshLP: _IvoryDispatch = 408;
pub const DispatchAshFP: _IvoryDispatch = 407;
pub const DispatchPointerDifferenceImmediate: _IvoryDispatch = 405;
pub const DispatchPointerDifferenceSP: _IvoryDispatch = 404;
pub const DispatchPointerDifferenceLP: _IvoryDispatch = 403;
pub const DispatchPointerDifferenceFP: _IvoryDispatch = 402;
pub const DispatchPointerPlusImmediate: _IvoryDispatch = 400;
pub const DispatchPointerPlusSP: _IvoryDispatch = 399;
pub const DispatchPointerPlusLP: _IvoryDispatch = 398;
pub const DispatchPointerPlusFP: _IvoryDispatch = 397;
pub const DispatchAssocImmediate: _IvoryDispatch = 395;
pub const DispatchAssocSP: _IvoryDispatch = 394;
pub const DispatchAssocLP: _IvoryDispatch = 393;
pub const DispatchAssocFP: _IvoryDispatch = 392;
pub const DispatchMemberImmediate: _IvoryDispatch = 390;
pub const DispatchMemberSP: _IvoryDispatch = 389;
pub const DispatchMemberLP: _IvoryDispatch = 388;
pub const DispatchMemberFP: _IvoryDispatch = 387;
pub const DispatchRgetfImmediate: _IvoryDispatch = 385;
pub const DispatchRgetfSP: _IvoryDispatch = 384;
pub const DispatchRgetfLP: _IvoryDispatch = 383;
pub const DispatchRgetfFP: _IvoryDispatch = 382;
pub const DispatchStackBltImmediate: _IvoryDispatch = 380;
pub const DispatchStackBltSP: _IvoryDispatch = 379;
pub const DispatchStackBltLP: _IvoryDispatch = 378;
pub const DispatchStackBltFP: _IvoryDispatch = 377;
pub const DispatchLshcBignumStepImmediate: _IvoryDispatch = 375;
pub const DispatchLshcBignumStepSP: _IvoryDispatch = 374;
pub const DispatchLshcBignumStepLP: _IvoryDispatch = 373;
pub const DispatchLshcBignumStepFP: _IvoryDispatch = 372;
pub const DispatchMultiplyDoubleImmediate: _IvoryDispatch = 370;
pub const DispatchMultiplyDoubleSP: _IvoryDispatch = 369;
pub const DispatchMultiplyDoubleLP: _IvoryDispatch = 368;
pub const DispatchMultiplyDoubleFP: _IvoryDispatch = 367;
pub const DispatchLshImmediate: _IvoryDispatch = 365;
pub const DispatchLshSP: _IvoryDispatch = 364;
pub const DispatchLshLP: _IvoryDispatch = 363;
pub const DispatchLshFP: _IvoryDispatch = 362;
pub const DispatchRotImmediate: _IvoryDispatch = 360;
pub const DispatchRotSP: _IvoryDispatch = 359;
pub const DispatchRotLP: _IvoryDispatch = 358;
pub const DispatchRotFP: _IvoryDispatch = 357;
pub const DispatchLogiorImmediate: _IvoryDispatch = 355;
pub const DispatchLogiorSP: _IvoryDispatch = 354;
pub const DispatchLogiorLP: _IvoryDispatch = 353;
pub const DispatchLogiorFP: _IvoryDispatch = 352;
pub const DispatchLogxorImmediate: _IvoryDispatch = 350;
pub const DispatchLogxorSP: _IvoryDispatch = 349;
pub const DispatchLogxorLP: _IvoryDispatch = 348;
pub const DispatchLogxorFP: _IvoryDispatch = 347;
pub const DispatchLogandImmediate: _IvoryDispatch = 345;
pub const DispatchLogandSP: _IvoryDispatch = 344;
pub const DispatchLogandLP: _IvoryDispatch = 343;
pub const DispatchLogandFP: _IvoryDispatch = 342;
pub const DispatchAluImmediate: _IvoryDispatch = 340;
pub const DispatchAluSP: _IvoryDispatch = 339;
pub const DispatchAluLP: _IvoryDispatch = 338;
pub const DispatchAluFP: _IvoryDispatch = 337;
pub const DispatchMaxImmediate: _IvoryDispatch = 335;
pub const DispatchMaxSP: _IvoryDispatch = 334;
pub const DispatchMaxLP: _IvoryDispatch = 333;
pub const DispatchMaxFP: _IvoryDispatch = 332;
pub const DispatchMinImmediate: _IvoryDispatch = 330;
pub const DispatchMinSP: _IvoryDispatch = 329;
pub const DispatchMinLP: _IvoryDispatch = 328;
pub const DispatchMinFP: _IvoryDispatch = 327;
pub const DispatchRationalQuotientImmediate: _IvoryDispatch = 325;
pub const DispatchRationalQuotientSP: _IvoryDispatch = 324;
pub const DispatchRationalQuotientLP: _IvoryDispatch = 323;
pub const DispatchRationalQuotientFP: _IvoryDispatch = 322;
pub const DispatchRoundImmediate: _IvoryDispatch = 320;
pub const DispatchRoundSP: _IvoryDispatch = 319;
pub const DispatchRoundLP: _IvoryDispatch = 318;
pub const DispatchRoundFP: _IvoryDispatch = 317;
pub const DispatchTruncateImmediate: _IvoryDispatch = 315;
pub const DispatchTruncateSP: _IvoryDispatch = 314;
pub const DispatchTruncateLP: _IvoryDispatch = 313;
pub const DispatchTruncateFP: _IvoryDispatch = 312;
pub const DispatchFloorImmediate: _IvoryDispatch = 310;
pub const DispatchFloorSP: _IvoryDispatch = 309;
pub const DispatchFloorLP: _IvoryDispatch = 308;
pub const DispatchFloorFP: _IvoryDispatch = 307;
pub const DispatchCeilingImmediate: _IvoryDispatch = 305;
pub const DispatchCeilingSP: _IvoryDispatch = 304;
pub const DispatchCeilingLP: _IvoryDispatch = 303;
pub const DispatchCeilingFP: _IvoryDispatch = 302;
pub const DispatchQuotientImmediate: _IvoryDispatch = 300;
pub const DispatchQuotientSP: _IvoryDispatch = 299;
pub const DispatchQuotientLP: _IvoryDispatch = 298;
pub const DispatchQuotientFP: _IvoryDispatch = 297;
pub const DispatchMultiplyImmediate: _IvoryDispatch = 295;
pub const DispatchMultiplySP: _IvoryDispatch = 294;
pub const DispatchMultiplyLP: _IvoryDispatch = 293;
pub const DispatchMultiplyFP: _IvoryDispatch = 292;
pub const DispatchRplacdImmediate: _IvoryDispatch = 290;
pub const DispatchRplacdSP: _IvoryDispatch = 289;
pub const DispatchRplacdLP: _IvoryDispatch = 288;
pub const DispatchRplacdFP: _IvoryDispatch = 287;
pub const DispatchRplacaImmediate: _IvoryDispatch = 285;
pub const DispatchRplacaSP: _IvoryDispatch = 284;
pub const DispatchRplacaLP: _IvoryDispatch = 283;
pub const DispatchRplacaFP: _IvoryDispatch = 282;
pub const DispatchEntryRestNotAccepted: _IvoryDispatch = 281;
pub const DispatchEntryRestAccepted: _IvoryDispatch = 280;
pub const DispatchLoopDecrementTos: _IvoryDispatch = 279;
pub const DispatchBranch: _IvoryDispatch = 278;
pub const DispatchPTagLdb: _IvoryDispatch = 277;
pub const DispatchPLdb: _IvoryDispatch = 276;
pub const DispatchCharLdb: _IvoryDispatch = 275;
pub const DispatchLdb: _IvoryDispatch = 274;
pub const DispatchBlock3ReadAluImmediate: _IvoryDispatch = 272;
pub const DispatchBlock3ReadAluSP: _IvoryDispatch = 271;
pub const DispatchBlock3ReadAluLP: _IvoryDispatch = 270;
pub const DispatchBlock3ReadAluFP: _IvoryDispatch = 269;
pub const DispatchBlock2ReadAluImmediate: _IvoryDispatch = 267;
pub const DispatchBlock2ReadAluSP: _IvoryDispatch = 266;
pub const DispatchBlock2ReadAluLP: _IvoryDispatch = 265;
pub const DispatchBlock2ReadAluFP: _IvoryDispatch = 264;
pub const DispatchBlock1ReadAluImmediate: _IvoryDispatch = 262;
pub const DispatchBlock1ReadAluSP: _IvoryDispatch = 261;
pub const DispatchBlock1ReadAluLP: _IvoryDispatch = 260;
pub const DispatchBlock1ReadAluFP: _IvoryDispatch = 259;
pub const DispatchBlock0ReadAluImmediate: _IvoryDispatch = 257;
pub const DispatchBlock0ReadAluSP: _IvoryDispatch = 256;
pub const DispatchBlock0ReadAluLP: _IvoryDispatch = 255;
pub const DispatchBlock0ReadAluFP: _IvoryDispatch = 254;
pub const DispatchCoprocessorWrite: _IvoryDispatch = 253;
pub const DispatchCoprocessorRead: _IvoryDispatch = 252;
pub const DispatchWriteInternalRegister: _IvoryDispatch = 251;
pub const DispatchReadInternalRegister: _IvoryDispatch = 250;
pub const DispatchSetSpToAddressSaveTosImmediate: _IvoryDispatch = 248;
pub const DispatchSetSpToAddressSaveTosSP: _IvoryDispatch = 247;
pub const DispatchSetSpToAddressSaveTosLP: _IvoryDispatch = 246;
pub const DispatchSetSpToAddressSaveTosFP: _IvoryDispatch = 245;
pub const DispatchSetSpToAddressImmediate: _IvoryDispatch = 243;
pub const DispatchSetSpToAddressSP: _IvoryDispatch = 242;
pub const DispatchSetSpToAddressLP: _IvoryDispatch = 241;
pub const DispatchSetSpToAddressFP: _IvoryDispatch = 240;
pub const DispatchPushAddressImmediate: _IvoryDispatch = 238;
pub const DispatchPushAddressSP: _IvoryDispatch = 237;
pub const DispatchPushAddressLP: _IvoryDispatch = 236;
pub const DispatchPushAddressFP: _IvoryDispatch = 235;
pub const DispatchSetCdrCode2Immediate: _IvoryDispatch = 233;
pub const DispatchSetCdrCode2SP: _IvoryDispatch = 232;
pub const DispatchSetCdrCode2LP: _IvoryDispatch = 231;
pub const DispatchSetCdrCode2FP: _IvoryDispatch = 230;
pub const DispatchSetCdrCode1Immediate: _IvoryDispatch = 228;
pub const DispatchSetCdrCode1SP: _IvoryDispatch = 227;
pub const DispatchSetCdrCode1LP: _IvoryDispatch = 226;
pub const DispatchSetCdrCode1FP: _IvoryDispatch = 225;
pub const DispatchPointerIncrementImmediate: _IvoryDispatch = 223;
pub const DispatchPointerIncrementSP: _IvoryDispatch = 222;
pub const DispatchPointerIncrementLP: _IvoryDispatch = 221;
pub const DispatchPointerIncrementFP: _IvoryDispatch = 220;
pub const DispatchDecrementImmediate: _IvoryDispatch = 218;
pub const DispatchDecrementSP: _IvoryDispatch = 217;
pub const DispatchDecrementLP: _IvoryDispatch = 216;
pub const DispatchDecrementFP: _IvoryDispatch = 215;
pub const DispatchIncrementImmediate: _IvoryDispatch = 213;
pub const DispatchIncrementSP: _IvoryDispatch = 212;
pub const DispatchIncrementLP: _IvoryDispatch = 211;
pub const DispatchIncrementFP: _IvoryDispatch = 210;
pub const DispatchSetToCdrPushCarImmediate: _IvoryDispatch = 208;
pub const DispatchSetToCdrPushCarSP: _IvoryDispatch = 207;
pub const DispatchSetToCdrPushCarLP: _IvoryDispatch = 206;
pub const DispatchSetToCdrPushCarFP: _IvoryDispatch = 205;
pub const DispatchSetToCdrImmediate: _IvoryDispatch = 203;
pub const DispatchSetToCdrSP: _IvoryDispatch = 202;
pub const DispatchSetToCdrLP: _IvoryDispatch = 201;
pub const DispatchSetToCdrFP: _IvoryDispatch = 200;
pub const DispatchSetToCarImmediate: _IvoryDispatch = 198;
pub const DispatchSetToCarSP: _IvoryDispatch = 197;
pub const DispatchSetToCarLP: _IvoryDispatch = 196;
pub const DispatchSetToCarFP: _IvoryDispatch = 195;
pub const DispatchFinishCallTosApply: _IvoryDispatch = 194;
pub const DispatchFinishCallTos: _IvoryDispatch = 193;
pub const DispatchFinishCallNApply: _IvoryDispatch = 192;
pub const DispatchFinishCallN: _IvoryDispatch = 191;
pub const DispatchBlock3ReadTest: _IvoryDispatch = 190;
pub const DispatchBlock2ReadTest: _IvoryDispatch = 189;
pub const DispatchBlock1ReadTest: _IvoryDispatch = 188;
pub const DispatchBlock0ReadTest: _IvoryDispatch = 187;
pub const DispatchBlock3ReadShift: _IvoryDispatch = 186;
pub const DispatchBlock2ReadShift: _IvoryDispatch = 185;
pub const DispatchBlock1ReadShift: _IvoryDispatch = 184;
pub const DispatchBlock0ReadShift: _IvoryDispatch = 183;
pub const DispatchBlock3Read: _IvoryDispatch = 182;
pub const DispatchBlock2Read: _IvoryDispatch = 181;
pub const DispatchBlock1Read: _IvoryDispatch = 180;
pub const DispatchBlock0Read: _IvoryDispatch = 179;
pub const DispatchMemoryReadAddress: _IvoryDispatch = 178;
pub const DispatchMemoryRead: _IvoryDispatch = 177;
pub const DispatchReturnSingleTOS: _IvoryDispatch = 176;
pub const DispatchReturnSingleT: _IvoryDispatch = 175;
pub const DispatchReturnSingleNIL: _IvoryDispatch = 174;
pub const DispatchUnaryMinusImmediate: _IvoryDispatch = 172;
pub const DispatchUnaryMinusSP: _IvoryDispatch = 171;
pub const DispatchUnaryMinusLP: _IvoryDispatch = 170;
pub const DispatchUnaryMinusFP: _IvoryDispatch = 169;
pub const DispatchPushAddressInstanceVariableOrdered: _IvoryDispatch = 168;
pub const DispatchPushInstanceVariableOrdered: _IvoryDispatch = 167;
pub const DispatchPushAddressInstanceVariable: _IvoryDispatch = 166;
pub const DispatchPushInstanceVariable: _IvoryDispatch = 165;
pub const DispatchUnbindNImmediate: _IvoryDispatch = 163;
pub const DispatchTakeValues: _IvoryDispatch = 162;
pub const DispatchReturnKludgeImmediate: _IvoryDispatch = 160;
pub const DispatchReturnKludgeSP: _IvoryDispatch = 159;
pub const DispatchReturnKludgeLP: _IvoryDispatch = 158;
pub const DispatchReturnKludgeFP: _IvoryDispatch = 157;
pub const DispatchReturnMultipleImmediate: _IvoryDispatch = 155;
pub const DispatchReturnMultipleSP: _IvoryDispatch = 154;
pub const DispatchReturnMultipleLP: _IvoryDispatch = 153;
pub const DispatchReturnMultipleFP: _IvoryDispatch = 152;
pub const DispatchPushLocalLogicVariablesImmediate: _IvoryDispatch = 150;
pub const DispatchPushLocalLogicVariablesSP: _IvoryDispatch = 149;
pub const DispatchPushLocalLogicVariablesLP: _IvoryDispatch = 148;
pub const DispatchPushLocalLogicVariablesFP: _IvoryDispatch = 147;
pub const DispatchPushAddressSpRelativeImmediate: _IvoryDispatch = 145;
pub const DispatchPushAddressSpRelativeSP: _IvoryDispatch = 144;
pub const DispatchPushAddressSpRelativeLP: _IvoryDispatch = 143;
pub const DispatchPushAddressSpRelativeFP: _IvoryDispatch = 142;
pub const DispatchPushNNils: _IvoryDispatch = 141;
pub const DispatchPushImmediate: _IvoryDispatch = 139;
pub const DispatchPushSP: _IvoryDispatch = 138;
pub const DispatchPushLP: _IvoryDispatch = 137;
pub const DispatchPushFP: _IvoryDispatch = 136;
pub const DispatchBranchFalseAndNoPopElseNoPopExtraPop: _IvoryDispatch = 135;
pub const DispatchBranchFalseElseNoPop: _IvoryDispatch = 134;
pub const DispatchBranchFalseAndNoPop: _IvoryDispatch = 133;
pub const DispatchBranchFalseNoPop: _IvoryDispatch = 132;
pub const DispatchBranchFalseExtraPop: _IvoryDispatch = 131;
pub const DispatchBranchFalseAndExtraPop: _IvoryDispatch = 130;
pub const DispatchBranchFalseElseExtraPop: _IvoryDispatch = 129;
pub const DispatchBranchFalse: _IvoryDispatch = 128;
pub const DispatchBranchTrueAndNoPopElseNoPopExtraPop: _IvoryDispatch = 127;
pub const DispatchBranchTrueElseNoPop: _IvoryDispatch = 126;
pub const DispatchBranchTrueAndNoPop: _IvoryDispatch = 125;
pub const DispatchBranchTrueNoPop: _IvoryDispatch = 124;
pub const DispatchBranchTrueExtraPop: _IvoryDispatch = 123;
pub const DispatchBranchTrueAndExtraPop: _IvoryDispatch = 122;
pub const DispatchBranchTrueElseExtraPop: _IvoryDispatch = 121;
pub const DispatchBranchTrue: _IvoryDispatch = 120;
pub const DispatchHalt: _IvoryDispatch = 119;
pub const DispatchNoOp: _IvoryDispatch = 118;
pub const DispatchPushGlobalLogicVariable: _IvoryDispatch = 117;
pub const DispatchCheckPreemptRequest: _IvoryDispatch = 116;
pub const DispatchMessageDispatch: _IvoryDispatch = 115;
pub const DispatchGenericDispatch: _IvoryDispatch = 114;
pub const DispatchCatchClose: _IvoryDispatch = 113;
pub const DispatchLocateLocals: _IvoryDispatch = 112;
pub const DispatchTypeMemberNoPop: _IvoryDispatch = 111;
pub const DispatchTypeMember: _IvoryDispatch = 110;
pub const DispatchPluspImmediate: _IvoryDispatch = 108;
pub const DispatchPluspSP: _IvoryDispatch = 107;
pub const DispatchPluspLP: _IvoryDispatch = 106;
pub const DispatchPluspFP: _IvoryDispatch = 105;
pub const DispatchMinuspImmediate: _IvoryDispatch = 103;
pub const DispatchMinuspSP: _IvoryDispatch = 102;
pub const DispatchMinuspLP: _IvoryDispatch = 101;
pub const DispatchMinuspFP: _IvoryDispatch = 100;
pub const DispatchZeropImmediate: _IvoryDispatch = 98;
pub const DispatchZeropSP: _IvoryDispatch = 97;
pub const DispatchZeropLP: _IvoryDispatch = 96;
pub const DispatchZeropFP: _IvoryDispatch = 95;
pub const DispatchBlock3WriteImmediate: _IvoryDispatch = 93;
pub const DispatchBlock3WriteSP: _IvoryDispatch = 92;
pub const DispatchBlock3WriteLP: _IvoryDispatch = 91;
pub const DispatchBlock3WriteFP: _IvoryDispatch = 90;
pub const DispatchBlock2WriteImmediate: _IvoryDispatch = 88;
pub const DispatchBlock2WriteSP: _IvoryDispatch = 87;
pub const DispatchBlock2WriteLP: _IvoryDispatch = 86;
pub const DispatchBlock2WriteFP: _IvoryDispatch = 85;
pub const DispatchBlock1WriteImmediate: _IvoryDispatch = 83;
pub const DispatchBlock1WriteSP: _IvoryDispatch = 82;
pub const DispatchBlock1WriteLP: _IvoryDispatch = 81;
pub const DispatchBlock1WriteFP: _IvoryDispatch = 80;
pub const DispatchBlock0WriteImmediate: _IvoryDispatch = 78;
pub const DispatchBlock0WriteSP: _IvoryDispatch = 77;
pub const DispatchBlock0WriteLP: _IvoryDispatch = 76;
pub const DispatchBlock0WriteFP: _IvoryDispatch = 75;
pub const DispatchPushLexicalVarImmediate: _IvoryDispatch = 73;
pub const DispatchPushLexicalVarSP: _IvoryDispatch = 72;
pub const DispatchPushLexicalVarLP: _IvoryDispatch = 71;
pub const DispatchPushLexicalVarFP: _IvoryDispatch = 70;
pub const DispatchProcBreakpointImmediate: _IvoryDispatch = 68;
pub const DispatchProcBreakpointSP: _IvoryDispatch = 67;
pub const DispatchProcBreakpointLP: _IvoryDispatch = 66;
pub const DispatchProcBreakpointFP: _IvoryDispatch = 65;
pub const DispatchLogicTailTestImmediate: _IvoryDispatch = 63;
pub const DispatchLogicTailTestSP: _IvoryDispatch = 62;
pub const DispatchLogicTailTestLP: _IvoryDispatch = 61;
pub const DispatchLogicTailTestFP: _IvoryDispatch = 60;
pub const DispatchDereferenceImmediate: _IvoryDispatch = 58;
pub const DispatchDereferenceSP: _IvoryDispatch = 57;
pub const DispatchDereferenceLP: _IvoryDispatch = 56;
pub const DispatchDereferenceFP: _IvoryDispatch = 55;
pub const DispatchTagImmediate: _IvoryDispatch = 53;
pub const DispatchTagSP: _IvoryDispatch = 52;
pub const DispatchTagLP: _IvoryDispatch = 51;
pub const DispatchTagFP: _IvoryDispatch = 50;
pub const DispatchJumpImmediate: _IvoryDispatch = 48;
pub const DispatchJumpSP: _IvoryDispatch = 47;
pub const DispatchJumpLP: _IvoryDispatch = 46;
pub const DispatchJumpFP: _IvoryDispatch = 45;
pub const DispatchStartCallImmediate: _IvoryDispatch = 43;
pub const DispatchStartCallSP: _IvoryDispatch = 42;
pub const DispatchStartCallLP: _IvoryDispatch = 41;
pub const DispatchStartCallFP: _IvoryDispatch = 40;
pub const DispatchEphemeralpImmediate: _IvoryDispatch = 38;
pub const DispatchEphemeralpSP: _IvoryDispatch = 37;
pub const DispatchEphemeralpLP: _IvoryDispatch = 36;
pub const DispatchEphemeralpFP: _IvoryDispatch = 35;
pub const DispatchRestoreBindingStackImmediate: _IvoryDispatch = 33;
pub const DispatchRestoreBindingStackSP: _IvoryDispatch = 32;
pub const DispatchRestoreBindingStackLP: _IvoryDispatch = 31;
pub const DispatchRestoreBindingStackFP: _IvoryDispatch = 30;
pub const DispatchBindLocativeImmediate: _IvoryDispatch = 28;
pub const DispatchBindLocativeSP: _IvoryDispatch = 27;
pub const DispatchBindLocativeLP: _IvoryDispatch = 26;
pub const DispatchBindLocativeFP: _IvoryDispatch = 25;
pub const DispatchSetupForce1dArrayImmediate: _IvoryDispatch = 23;
pub const DispatchSetupForce1dArraySP: _IvoryDispatch = 22;
pub const DispatchSetupForce1dArrayLP: _IvoryDispatch = 21;
pub const DispatchSetupForce1dArrayFP: _IvoryDispatch = 20;
pub const DispatchSetup1dArrayImmediate: _IvoryDispatch = 18;
pub const DispatchSetup1dArraySP: _IvoryDispatch = 17;
pub const DispatchSetup1dArrayLP: _IvoryDispatch = 16;
pub const DispatchSetup1dArrayFP: _IvoryDispatch = 15;
pub const DispatchEndpImmediate: _IvoryDispatch = 13;
pub const DispatchEndpSP: _IvoryDispatch = 12;
pub const DispatchEndpLP: _IvoryDispatch = 11;
pub const DispatchEndpFP: _IvoryDispatch = 10;
pub const DispatchCdrImmediate: _IvoryDispatch = 8;
pub const DispatchCdrSP: _IvoryDispatch = 7;
pub const DispatchCdrLP: _IvoryDispatch = 6;
pub const DispatchCdrFP: _IvoryDispatch = 5;
pub const DispatchCarImmediate: _IvoryDispatch = 3;
pub const DispatchCarSP: _IvoryDispatch = 2;
pub const DispatchCarLP: _IvoryDispatch = 1;
pub const DispatchCarFP: _IvoryDispatch = 0;
pub const DispatchPushPackedInstruction77: _IvoryDispatch = 721;
pub const DispatchPushPackedInstruction76: _IvoryDispatch = 720;
pub const DispatchPushPackedInstruction75: _IvoryDispatch = 719;
pub const DispatchPushPackedInstruction74: _IvoryDispatch = 718;
pub const DispatchPushPackedInstruction73: _IvoryDispatch = 717;
pub const DispatchPushPackedInstruction72: _IvoryDispatch = 716;
pub const DispatchPushPackedInstruction71: _IvoryDispatch = 715;
pub const DispatchPushPackedInstruction70: _IvoryDispatch = 714;
pub const DispatchPushPackedInstruction67: _IvoryDispatch = 713;
pub const DispatchPushPackedInstruction66: _IvoryDispatch = 712;
pub const DispatchPushPackedInstruction65: _IvoryDispatch = 711;
pub const DispatchPushPackedInstruction64: _IvoryDispatch = 710;
pub const DispatchPushPackedInstruction63: _IvoryDispatch = 709;
pub const DispatchPushPackedInstruction62: _IvoryDispatch = 708;
pub const DispatchPushPackedInstruction61: _IvoryDispatch = 707;
pub const DispatchPushPackedInstruction60: _IvoryDispatch = 706;
pub const DispatchCallGenericPrefetch: _IvoryDispatch = 705;
pub const DispatchCallIndirectPrefetch: _IvoryDispatch = 704;
pub const DispatchCallCompiledOddPrefetch: _IvoryDispatch = 703;
pub const DispatchCallCompiledEvenPrefetch: _IvoryDispatch = 702;
pub const DispatchCallGeneric: _IvoryDispatch = 701;
pub const DispatchCallIndirect: _IvoryDispatch = 700;
pub const DispatchCallCompiledOdd: _IvoryDispatch = 699;
pub const DispatchCallCompiledEven: _IvoryDispatch = 698;
pub const DispatchPushOddPc: _IvoryDispatch = 697;
pub const DispatchPushEvenPc: _IvoryDispatch = 696;
pub const DispatchPushGcForward: _IvoryDispatch = 695;
pub const DispatchPushLogicVariable: _IvoryDispatch = 694;
pub const DispatchPushCharacter: _IvoryDispatch = 693;
pub const DispatchPushBoundLocation: _IvoryDispatch = 692;
pub const DispatchPushSpareImmediate1: _IvoryDispatch = 691;
pub const DispatchPushPhysicalAddress: _IvoryDispatch = 690;
pub const DispatchPushSparePointer2: _IvoryDispatch = 689;
pub const DispatchPushSparePointer1: _IvoryDispatch = 688;
pub const DispatchPushGenericFunction: _IvoryDispatch = 687;
pub const DispatchPushCompiledFunction: _IvoryDispatch = 686;
pub const DispatchPushDynamicClosure: _IvoryDispatch = 685;
pub const DispatchPushLexicalClosure: _IvoryDispatch = 684;
pub const DispatchPushLocative: _IvoryDispatch = 683;
pub const DispatchPushSymbol: _IvoryDispatch = 682;
pub const DispatchPushString: _IvoryDispatch = 681;
pub const DispatchPushArray: _IvoryDispatch = 680;
pub const DispatchPushList: _IvoryDispatch = 679;
pub const DispatchPushNil: _IvoryDispatch = 678;
pub const DispatchPushStringInstance: _IvoryDispatch = 677;
pub const DispatchPushArrayInstance: _IvoryDispatch = 676;
pub const DispatchPushListInstance: _IvoryDispatch = 675;
pub const DispatchPushInstance: _IvoryDispatch = 674;
pub const DispatchPushSpareNumber: _IvoryDispatch = 673;
pub const DispatchPushComplex: _IvoryDispatch = 672;
pub const DispatchPushBigRatio: _IvoryDispatch = 671;
pub const DispatchPushBignum: _IvoryDispatch = 670;
pub const DispatchPushDoubleFloat: _IvoryDispatch = 669;
pub const DispatchPushSingleFloat: _IvoryDispatch = 668;
pub const DispatchPushSmallRatio: _IvoryDispatch = 667;
pub const DispatchPushFixnum: _IvoryDispatch = 666;
pub const DispatchPushElementForward: _IvoryDispatch = 665;
pub const DispatchPushHeaderForward: _IvoryDispatch = 664;
pub const DispatchPushOneQForward: _IvoryDispatch = 663;
pub const DispatchPushExternalValueCellPointer: _IvoryDispatch = 662;
pub const DispatchPushHeaderI: _IvoryDispatch = 661;
pub const DispatchPushHeaderP: _IvoryDispatch = 660;
pub const DispatchPushMonitorForward: _IvoryDispatch = 659;
pub const DispatchPushNull: _IvoryDispatch = 658;
pub const TypeEvenPC: _IvoryType = 38;
pub type _IvoryDispatch = libc::c_uint;
pub type _IvoryType = libc::c_uint;
pub const TypePackedInstruction77: _IvoryType = 63;
pub const TypePackedInstruction76: _IvoryType = 62;
pub const TypePackedInstruction75: _IvoryType = 61;
pub const TypePackedInstruction74: _IvoryType = 60;
pub const TypePackedInstruction73: _IvoryType = 59;
pub const TypePackedInstruction72: _IvoryType = 58;
pub const TypePackedInstruction71: _IvoryType = 57;
pub const TypePackedInstruction70: _IvoryType = 56;
pub const TypePackedInstruction67: _IvoryType = 55;
pub const TypePackedInstruction66: _IvoryType = 54;
pub const TypePackedInstruction65: _IvoryType = 53;
pub const TypePackedInstruction64: _IvoryType = 52;
pub const TypePackedInstruction63: _IvoryType = 51;
pub const TypePackedInstruction62: _IvoryType = 50;
pub const TypePackedInstruction61: _IvoryType = 49;
pub const TypePackedInstruction60: _IvoryType = 48;
pub const TypeCallGenericPrefetch: _IvoryType = 47;
pub const TypeCallIndirectPrefetch: _IvoryType = 46;
pub const TypeCallCompiledOddPrefetch: _IvoryType = 45;
pub const TypeCallCompiledEvenPrefetch: _IvoryType = 44;
pub const TypeCallGeneric: _IvoryType = 43;
pub const TypeCallIndirect: _IvoryType = 42;
pub const TypeCallCompiledOdd: _IvoryType = 41;
pub const TypeCallCompiledEven: _IvoryType = 40;
pub const TypeGCForward: _IvoryType = 37;
pub const TypeLogicVariable: _IvoryType = 36;
pub const TypeCharacter: _IvoryType = 35;
pub const TypeBoundLocation: _IvoryType = 34;
pub const TypeSpareImmediate1: _IvoryType = 33;
pub const TypePhysicalAddress: _IvoryType = 32;
pub const TypeSparePointer2: _IvoryType = 31;
pub const TypeSparePointer1: _IvoryType = 30;
pub const TypeGenericFunction: _IvoryType = 29;
pub const TypeCompiledFunction: _IvoryType = 28;
pub const TypeDynamicClosure: _IvoryType = 27;
pub const TypeLexicalClosure: _IvoryType = 26;
pub const TypeLocative: _IvoryType = 25;
pub const TypeSymbol: _IvoryType = 24;
pub const TypeString: _IvoryType = 23;
pub const TypeArray: _IvoryType = 22;
pub const TypeList: _IvoryType = 21;
pub const TypeNIL: _IvoryType = 20;
pub const TypeStringInstance: _IvoryType = 19;
pub const TypeArrayInstance: _IvoryType = 18;
pub const TypeListInstance: _IvoryType = 17;
pub const TypeInstance: _IvoryType = 16;
pub const TypeSpareNumber: _IvoryType = 15;
pub const TypeComplex: _IvoryType = 14;
pub const TypeBigRatio: _IvoryType = 13;
pub const TypeBignum: _IvoryType = 12;
pub const TypeDoubleFloat: _IvoryType = 11;
pub const TypeSingleFloat: _IvoryType = 10;
pub const TypeSmallRatio: _IvoryType = 9;
pub const TypeFixnum: _IvoryType = 8;
pub const TypeElementForward: _IvoryType = 7;
pub const TypeHeaderForward: _IvoryType = 6;
pub const TypeOneQForward: _IvoryType = 5;
pub const TypeExternalValueCellPointer: _IvoryType = 4;
pub const TypeHeaderI: _IvoryType = 3;
pub const TypeHeaderP: _IvoryType = 2;
pub const TypeMonitorForward: _IvoryType = 1;
pub const TypeNull: _IvoryType = 0;
static mut ReturnInstructionDecoder: [u32; 3] = [
    DispatchReturnSingleNIL,
    DispatchReturnSingleT,
    DispatchReturnSingleTOS,
];
static mut WordInstructionDecoder: [u32; 64] = [
    DispatchPushNull,
    DispatchPushMonitorForward,
    DispatchPushHeaderP,
    DispatchPushHeaderI,
    DispatchPushExternalValueCellPointer,
    DispatchPushOneQForward,
    DispatchPushHeaderForward,
    DispatchPushElementForward,
    DispatchPushFixnum,
    DispatchPushSmallRatio,
    DispatchPushSingleFloat,
    DispatchPushDoubleFloat,
    DispatchPushBignum,
    DispatchPushBigRatio,
    DispatchPushComplex,
    DispatchPushSpareNumber,
    DispatchPushInstance,
    DispatchPushListInstance,
    DispatchPushArrayInstance,
    DispatchPushStringInstance,
    DispatchPushNil,
    DispatchPushList,
    DispatchPushArray,
    DispatchPushString,
    DispatchPushSymbol,
    DispatchPushLocative,
    DispatchPushLexicalClosure,
    DispatchPushDynamicClosure,
    DispatchPushCompiledFunction,
    DispatchPushGenericFunction,
    DispatchPushSparePointer1,
    DispatchPushSparePointer2,
    DispatchPushPhysicalAddress,
    DispatchPushSpareImmediate1,
    DispatchPushBoundLocation,
    DispatchPushCharacter,
    DispatchPushLogicVariable,
    DispatchPushGcForward,
    DispatchPushEvenPc,
    DispatchPushOddPc,
    DispatchCallCompiledEven,
    DispatchCallCompiledOdd,
    DispatchCallIndirect,
    DispatchCallGeneric,
    DispatchCallCompiledEvenPrefetch,
    DispatchCallCompiledOddPrefetch,
    DispatchCallIndirectPrefetch,
    DispatchCallGenericPrefetch,
    DispatchPushPackedInstruction60,
    DispatchPushPackedInstruction61,
    DispatchPushPackedInstruction62,
    DispatchPushPackedInstruction63,
    DispatchPushPackedInstruction64,
    DispatchPushPackedInstruction65,
    DispatchPushPackedInstruction66,
    DispatchPushPackedInstruction67,
    DispatchPushPackedInstruction70,
    DispatchPushPackedInstruction71,
    DispatchPushPackedInstruction72,
    DispatchPushPackedInstruction73,
    DispatchPushPackedInstruction74,
    DispatchPushPackedInstruction75,
    DispatchPushPackedInstruction76,
    DispatchPushPackedInstruction77,
];
static mut PopInstructionDecoder: [u32; 256] = [
    DispatchCarPop,
    DispatchCdrPop,
    DispatchEndpPop,
    DispatchSetup1dArrayPop,
    DispatchSetupForce1dArrayPop,
    DispatchBindLocativePop,
    DispatchRestoreBindingStackPop,
    DispatchEphemeralpPop,
    DispatchStartCallPop,
    DispatchJumpPop,
    DispatchTagPop,
    DispatchDereferencePop,
    DispatchLogicTailTestPop,
    DispatchProcBreakpointPop,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchPushLexicalVarPop,
    DispatchPushLexicalVarPop,
    DispatchPushLexicalVarPop,
    DispatchPushLexicalVarPop,
    DispatchPushLexicalVarPop,
    DispatchPushLexicalVarPop,
    DispatchPushLexicalVarPop,
    DispatchPushLexicalVarPop,
    DispatchBlock0WritePop,
    DispatchBlock1WritePop,
    DispatchBlock2WritePop,
    DispatchBlock3WritePop,
    DispatchZeropPop,
    DispatchMinuspPop,
    DispatchPluspPop,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchPushPop,
    DispatchIllegalInstruction,
    DispatchPushAddressSpRelativePop,
    DispatchPushLocalLogicVariablesPop,
    DispatchReturnMultiplePop,
    DispatchReturnKludgePop,
    DispatchIllegalInstruction,
    DispatchUnbindNPop,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchUnaryMinusPop,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchSetToCarPop,
    DispatchSetToCdrPop,
    DispatchSetToCdrPushCarPop,
    DispatchIncrementPop,
    DispatchDecrementPop,
    DispatchPointerIncrementPop,
    DispatchSetCdrCode1Pop,
    DispatchSetCdrCode2Pop,
    DispatchPushAddressPop,
    DispatchSetSpToAddressPop,
    DispatchSetSpToAddressSaveTosPop,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchBlock0ReadAluPop,
    DispatchBlock1ReadAluPop,
    DispatchBlock2ReadAluPop,
    DispatchBlock3ReadAluPop,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchRplacaPop,
    DispatchRplacdPop,
    DispatchMultiplyPop,
    DispatchQuotientPop,
    DispatchCeilingPop,
    DispatchFloorPop,
    DispatchTruncatePop,
    DispatchRoundPop,
    DispatchIllegalInstruction,
    DispatchRationalQuotientPop,
    DispatchMinPop,
    DispatchMaxPop,
    DispatchAluPop,
    DispatchLogandPop,
    DispatchLogxorPop,
    DispatchLogiorPop,
    DispatchRotPop,
    DispatchLshPop,
    DispatchMultiplyDoublePop,
    DispatchLshcBignumStepPop,
    DispatchStackBltPop,
    DispatchRgetfPop,
    DispatchMemberPop,
    DispatchAssocPop,
    DispatchPointerPlusPop,
    DispatchPointerDifferencePop,
    DispatchAshPop,
    DispatchStoreConditionalPop,
    DispatchMemoryWritePop,
    DispatchPStoreContentsPop,
    DispatchBindLocativeToValuePop,
    DispatchUnifyPop,
    DispatchPopLexicalVarPop,
    DispatchPopLexicalVarPop,
    DispatchPopLexicalVarPop,
    DispatchPopLexicalVarPop,
    DispatchPopLexicalVarPop,
    DispatchPopLexicalVarPop,
    DispatchPopLexicalVarPop,
    DispatchPopLexicalVarPop,
    DispatchMovemLexicalVarPop,
    DispatchMovemLexicalVarPop,
    DispatchMovemLexicalVarPop,
    DispatchMovemLexicalVarPop,
    DispatchMovemLexicalVarPop,
    DispatchMovemLexicalVarPop,
    DispatchMovemLexicalVarPop,
    DispatchMovemLexicalVarPop,
    DispatchEqualNumberPop,
    DispatchLesspPop,
    DispatchGreaterpPop,
    DispatchEqlPop,
    DispatchEqualNumberNoPopPop,
    DispatchLesspNoPopPop,
    DispatchGreaterpNoPopPop,
    DispatchEqlNoPopPop,
    DispatchEqPop,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchLogtestPop,
    DispatchEqNoPopPop,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchLogtestNoPopPop,
    DispatchAddPop,
    DispatchSubPop,
    Dispatch32BitPlusPop,
    Dispatch32BitDifferencePop,
    DispatchAddBignumStepPop,
    DispatchSubBignumStepPop,
    DispatchMultiplyBignumStepPop,
    DispatchDivideBignumStepPop,
    DispatchAset1Pop,
    DispatchAllocateListBlockPop,
    DispatchAref1Pop,
    DispatchAloc1Pop,
    DispatchStoreArrayLeaderPop,
    DispatchAllocateStructureBlockPop,
    DispatchArrayLeaderPop,
    DispatchAlocLeaderPop,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchInstanceRefPop,
    DispatchInstanceSetPop,
    DispatchInstanceLocPop,
    DispatchSetTagPop,
    DispatchIllegalInstruction,
    DispatchUnsignedLesspPop,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchUnsignedLesspNoPopPop,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchPopPop,
    DispatchMovemPop,
    DispatchMergeCdrNoPopPop,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchFastAref1Pop,
    DispatchFastAset1Pop,
    DispatchStackBltAddressPop,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
    DispatchIllegalInstruction,
];
static mut IllegalInstructionDecoder: usize = DispatchIllegalInstruction
   ;
 fn DecodeNoneFunction(
    mut i: u32,
    mut cp: *mut InstructionCacheLine,
) {
    (*cp).operand = 0;
}
static mut InstructionCacheLookupCPRepresentation: InstructionCacheLine = {
    let mut init = _InstructionCacheLine {
        pc: LispObj {
            parts: {
                let mut init = _LispObj {
                    tag: TypeOddPC as usize as ui32,
                    data: C2RustUnnamed {
                        u: -(1) as ui32,
                    },
                };
                init
            },
        },
        next_pc: LispObj {
            parts: {
                let mut init = _LispObj {
                    tag: TypeOddPC as usize as ui32,
                    data: C2RustUnnamed {
                        u: -(1) as ui32,
                    },
                };
                init
            },
        },
        code: DispatchInstructionCacheLookup,
        operand: 0,
        instruction: 0 as usize as libc::c_uint,
        next_cp: 0 as *const _InstructionCacheLine as *mut _InstructionCacheLine,
    };
    init
};
 fn Decode8BitUnsignedOperandFunction(
    mut i: u32,
    mut cp: *mut InstructionCacheLine,
) {
    (*cp).operand = i & 0o377;
}
 fn Decode8BitSignedOperandFunction(
    mut i: u32,
    mut cp: *mut InstructionCacheLine,
) {
    (*cp)
        .operand = ((i as libc::c_uint) << 24)
        / 16777216;
}
 fn Decode10BitUnsignedOperandFunction(
    mut i: u32,
    mut cp: *mut InstructionCacheLine,
) {
    (*cp).operand = i & 0o1777;
}
 fn Decode12BitUnsignedOperandFunction(
    mut i: u32,
    mut cp: *mut InstructionCacheLine,
) {
    (*cp).operand = i & 0o7777;
}
 fn DecodeFPOperandFunction(
    mut i: u32,
    mut cp: *mut InstructionCacheLine,
) {
    (*cp).operand = i & 0o377;
}
 fn DecodeLPOperandFunction(
    mut i: u32,
    mut cp: *mut InstructionCacheLine,
) {
    (*cp).operand = i & 0o377;
}
 fn DecodeSPOperandFunction(
    mut i: u32,
    mut cp: *mut InstructionCacheLine,
) {
    let mut offset: usize = i & 0o377;
    if offset == 0 as usize {
        (*cp)
            .code = PopInstructionDecoder[(i >> 10
            & ((1) << 8) - 1) as usize];
        (*cp).operand = 0;
    } else {
        (*cp).operand = offset - 255;
    };
}
 fn DecodeBranchOperandFunction(
    mut i: u32,
    mut cp: *mut InstructionCacheLine,
) {
    (*cp)
        .operand = ((i as libc::c_uint) << 22)
        / 4194304;
}
 fn DecodeReturnOperandFunction(
    mut i: u32,
    mut cp: *mut InstructionCacheLine,
) {
    match i & 0o1777 as usize {
        512 => {
            (*cp).code = ReturnInstructionDecoder[2 as usize as usize];
            (*cp).operand = 0;
        }
        544 => {
            (*cp).code = ReturnInstructionDecoder[0 as usize as usize];
            (*cp).operand = 0;
        }
        545 => {
            (*cp).code = ReturnInstructionDecoder[1 as usize as usize];
            (*cp).operand = 0;
        }
        _ => {}
    };
}
 fn DecodeEntryOperandFunction(
    mut i: u32,
    mut cp: *mut InstructionCacheLine,
) {
    (*cp)
        .operand = (i >> 18
        & ((1) << 8) - 1
        & ((1) << 8) - 1)
        << 8
        | i & 0o377
            & !((((1) << 8) - 1)
                << 8);
}
static mut PackedInstructionDecoder: [DecoderPair; 1024] = unsafe {
    [
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCdrFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCdrLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCdrSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCdrImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEndpFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEndpLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEndpSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEndpImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetup1dArrayFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetup1dArrayLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetup1dArraySP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetup1dArrayImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetupForce1dArrayFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetupForce1dArrayLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetupForce1dArraySP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetupForce1dArrayImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBindLocativeFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBindLocativeLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBindLocativeSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBindLocativeImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRestoreBindingStackFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRestoreBindingStackLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRestoreBindingStackSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRestoreBindingStackImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEphemeralpFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEphemeralpLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEphemeralpSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEphemeralpImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStartCallFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStartCallLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStartCallSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStartCallImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchJumpFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchJumpLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchJumpSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchJumpImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTagFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTagLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTagSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTagImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDereferenceFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDereferenceLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDereferenceSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDereferenceImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogicTailTestFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogicTailTestLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogicTailTestSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogicTailTestImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchProcBreakpointFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchProcBreakpointLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchProcBreakpointSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchProcBreakpointImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0WriteFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0WriteLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0WriteSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0WriteImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1WriteFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1WriteLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1WriteSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1WriteImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2WriteFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2WriteLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2WriteSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2WriteImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3WriteFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3WriteLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3WriteSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3WriteImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchZeropFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchZeropLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchZeropSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchZeropImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMinuspFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMinuspLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMinuspSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMinuspImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPluspFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPluspLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPluspSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPluspImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMember,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTypeMemberNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode12BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLocateLocals,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLocateLocals,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLocateLocals,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLocateLocals,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCatchClose,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCatchClose,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCatchClose,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCatchClose,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchGenericDispatch,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchGenericDispatch,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchGenericDispatch,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchGenericDispatch,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMessageDispatch,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMessageDispatch,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMessageDispatch,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMessageDispatch,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCheckPreemptRequest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCheckPreemptRequest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCheckPreemptRequest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCheckPreemptRequest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushGlobalLogicVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushGlobalLogicVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushGlobalLogicVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushGlobalLogicVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchNoOp,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchNoOp,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchNoOp,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchNoOp,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchHalt,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchHalt,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchHalt,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchHalt,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrue,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrue,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrue,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrue,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueElseExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueElseExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueElseExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueElseExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueAndExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueAndExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueAndExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueAndExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueAndNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueAndNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueAndNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueAndNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueElseNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueElseNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueElseNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueElseNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueAndNoPopElseNoPopExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueAndNoPopElseNoPopExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueAndNoPopElseNoPopExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchTrueAndNoPopElseNoPopExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalse,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalse,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalse,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalse,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseElseExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseElseExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseElseExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseElseExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseAndExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseAndExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseAndExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseAndExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseAndNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseAndNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseAndNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseAndNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseElseNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseElseNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseElseNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseElseNoPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseAndNoPopElseNoPopExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseAndNoPopElseNoPopExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseAndNoPopElseNoPopExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranchFalseAndNoPopElseNoPopExtraPop,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushNNils,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushNNils,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushNNils,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushNNils,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressSpRelativeFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressSpRelativeLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressSpRelativeSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressSpRelativeImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLocalLogicVariablesFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLocalLogicVariablesLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLocalLogicVariablesSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushLocalLogicVariablesImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReturnMultipleFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReturnMultipleLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReturnMultipleSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReturnMultipleImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReturnKludgeFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReturnKludgeLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReturnKludgeSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReturnKludgeImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTakeValues,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTakeValues,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTakeValues,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTakeValues,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnbindNImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnaryMinusFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnaryMinusLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnaryMinusSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnaryMinusImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReturnSingleTOS,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeReturnOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReturnSingleTOS,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeReturnOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReturnSingleTOS,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeReturnOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReturnSingleTOS,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeReturnOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemoryRead,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemoryRead,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemoryRead,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemoryRead,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemoryReadAddress,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemoryReadAddress,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemoryReadAddress,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemoryReadAddress,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3Read,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3ReadShift,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3ReadTest,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallN,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallN,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallN,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallN,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallNApply,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallNApply,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallNApply,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallNApply,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallTos,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallTos,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallTos,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallTos,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallTosApply,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallTosApply,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallTosApply,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFinishCallTosApply,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetToCarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetToCarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetToCarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetToCarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetToCdrFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetToCdrLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetToCdrSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetToCdrImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetToCdrPushCarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetToCdrPushCarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetToCdrPushCarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetToCdrPushCarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIncrementFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIncrementLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIncrementSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIncrementImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDecrementFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDecrementLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDecrementSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDecrementImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPointerIncrementFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPointerIncrementLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPointerIncrementSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPointerIncrementImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetCdrCode1FP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetCdrCode1LP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetCdrCode1SP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetCdrCode1Immediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetCdrCode2FP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetCdrCode2LP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetCdrCode2SP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetCdrCode2Immediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPushAddressImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetSpToAddressFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetSpToAddressLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetSpToAddressSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetSpToAddressImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetSpToAddressSaveTosFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetSpToAddressSaveTosLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetSpToAddressSaveTosSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetSpToAddressSaveTosImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReadInternalRegister,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReadInternalRegister,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReadInternalRegister,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchReadInternalRegister,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchWriteInternalRegister,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchWriteInternalRegister,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchWriteInternalRegister,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchWriteInternalRegister,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCoprocessorRead,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCoprocessorRead,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCoprocessorRead,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCoprocessorRead,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCoprocessorWrite,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCoprocessorWrite,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCoprocessorWrite,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCoprocessorWrite,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0ReadAluFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0ReadAluLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0ReadAluSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock0ReadAluImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1ReadAluFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1ReadAluLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1ReadAluSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock1ReadAluImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2ReadAluFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2ReadAluLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2ReadAluSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock2ReadAluImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3ReadAluFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3ReadAluLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3ReadAluSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBlock3ReadAluImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCharLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCharLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCharLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCharLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPTagLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPTagLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPTagLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPTagLdb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranch,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranch,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranch,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBranch,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLoopDecrementTos,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLoopDecrementTos,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLoopDecrementTos,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLoopDecrementTos,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEntryRestAccepted,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeEntryOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEntryRestAccepted,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeEntryOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEntryRestAccepted,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeEntryOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEntryRestAccepted,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeEntryOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEntryRestNotAccepted,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeEntryOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEntryRestNotAccepted,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeEntryOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEntryRestNotAccepted,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeEntryOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEntryRestNotAccepted,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeEntryOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRplacaFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRplacaLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRplacaSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRplacaImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRplacdFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRplacdLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRplacdSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRplacdImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMultiplyFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMultiplyLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMultiplySP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMultiplyImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchQuotientFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchQuotientLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchQuotientSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchQuotientImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCeilingFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCeilingLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCeilingSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCeilingImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFloorFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFloorLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFloorSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFloorImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTruncateFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTruncateLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTruncateSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchTruncateImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRoundFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRoundLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRoundSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRoundImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRationalQuotientFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRationalQuotientLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRationalQuotientSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRationalQuotientImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMinFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMinLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMinSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMinImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMaxFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMaxLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMaxSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMaxImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAluFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAluLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAluSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAluImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogandFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogandLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogandSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogandImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogxorFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogxorLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogxorSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogxorImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogiorFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogiorLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogiorSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogiorImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRotFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRotLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRotSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRotImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLshFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLshLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLshSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLshImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMultiplyDoubleFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMultiplyDoubleLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMultiplyDoubleSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMultiplyDoubleImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLshcBignumStepFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLshcBignumStepLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLshcBignumStepSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLshcBignumStepImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStackBltFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStackBltLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStackBltSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStackBltImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRgetfFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRgetfLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRgetfSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchRgetfImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemberFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemberLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemberSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemberImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAssocFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAssocLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAssocSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAssocImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPointerPlusFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPointerPlusLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPointerPlusSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPointerPlusImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPointerDifferenceFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPointerDifferenceLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPointerDifferenceSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPointerDifferenceImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAshFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAshLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAshSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAshImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStoreConditionalFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStoreConditionalLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStoreConditionalSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStoreConditionalImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemoryWriteFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemoryWriteLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemoryWriteSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMemoryWriteImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPStoreContentsFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPStoreContentsLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPStoreContentsSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPStoreContentsImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBindLocativeToValueFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBindLocativeToValueLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBindLocativeToValueSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchBindLocativeToValueImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnifyFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnifyLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnifySP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnifyImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLexicalVarImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqualNumberFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqualNumberLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqualNumberSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqualNumberImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLesspFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLesspLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLesspSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLesspImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchGreaterpFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchGreaterpLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchGreaterpSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchGreaterpImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqlFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqlLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqlSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqlImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqualNumberNoPopFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqualNumberNoPopLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqualNumberNoPopSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqualNumberNoPopImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLesspNoPopFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLesspNoPopLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLesspNoPopSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLesspNoPopImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchGreaterpNoPopFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchGreaterpNoPopLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchGreaterpNoPopSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchGreaterpNoPopImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqlNoPopFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqlNoPopLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqlNoPopSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqlNoPopImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogtestFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogtestLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogtestSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogtestImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqNoPopFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqNoPopLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqNoPopSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchEqNoPopImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogtestNoPopFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogtestNoPopLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogtestNoPopSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLogtestNoPopImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitSignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAddFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAddLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAddSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAddImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSubFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSubLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSubSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSubImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: Dispatch32BitPlusFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: Dispatch32BitPlusLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: Dispatch32BitPlusSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: Dispatch32BitPlusImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: Dispatch32BitDifferenceFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: Dispatch32BitDifferenceLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: Dispatch32BitDifferenceSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: Dispatch32BitDifferenceImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAddBignumStepFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAddBignumStepLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAddBignumStepSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAddBignumStepImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSubBignumStepFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSubBignumStepLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSubBignumStepSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSubBignumStepImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMultiplyBignumStepFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMultiplyBignumStepLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMultiplyBignumStepSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMultiplyBignumStepImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDivideBignumStepFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDivideBignumStepLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDivideBignumStepSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDivideBignumStepImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAset1FP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAset1LP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAset1SP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAset1Immediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAllocateListBlockFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAllocateListBlockLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAllocateListBlockSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAllocateListBlockImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAref1FP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAref1LP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAref1SP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAref1Immediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAloc1FP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAloc1LP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAloc1SP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAloc1Immediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStoreArrayLeaderFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStoreArrayLeaderLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStoreArrayLeaderSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStoreArrayLeaderImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAllocateStructureBlockFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAllocateStructureBlockLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAllocateStructureBlockSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAllocateStructureBlockImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchArrayLeaderFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchArrayLeaderLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchArrayLeaderSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchArrayLeaderImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAlocLeaderFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAlocLeaderLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAlocLeaderSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchAlocLeaderImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemInstanceVariable,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemInstanceVariableOrdered,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchInstanceRefFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchInstanceRefLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchInstanceRefSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchInstanceRefImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchInstanceSetFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchInstanceSetLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchInstanceSetSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchInstanceSetImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchInstanceLocFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchInstanceLocLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchInstanceLocSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchInstanceLocImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetTagFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetTagLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetTagSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchSetTagImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnsignedLesspFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnsignedLesspLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnsignedLesspSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnsignedLesspImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnsignedLesspNoPopFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnsignedLesspNoPopLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnsignedLesspNoPopSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchUnsignedLesspNoPopImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPopImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMovemImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMergeCdrNoPopFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMergeCdrNoPopLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMergeCdrNoPopSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchMergeCdrNoPopImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFastAref1FP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFastAref1LP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFastAref1SP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFastAref1Immediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFastAset1FP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFastAset1LP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFastAset1SP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchFastAset1Immediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStackBltAddressFP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeFPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStackBltAddressLP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeLPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStackBltAddressSP,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeSPOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchStackBltAddressImmediate,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCharDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCharDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCharDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCharDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPTagDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPTagDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPTagDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchPTagDpb,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchIllegalInstruction,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeNoneFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLoopIncrementTosLessThan,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLoopIncrementTosLessThan,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLoopIncrementTosLessThan,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchLoopIncrementTosLessThan,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        DecodeBranchOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCatchOpen,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCatchOpen,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCatchOpen,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchCatchOpen,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode8BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchHack,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchHack,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchHack,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
        {
            let mut init = _DecoderPair {
                dispatch: DispatchHack,
                decode: ::std::mem::transmute::<
                    Option::<
                        fn(
                            u32,
                            *mut InstructionCacheLine,
                        ) -> (),
                    >,
                    Option::<fn() -> ()>,
                >(
                    Some(
                        Decode10BitUnsignedOperandFunction
                            as fn(
                                u32,
                                *mut InstructionCacheLine,
                            ) -> (),
                    ),
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut ivory_dispatch_names: [*const libc::c_char; 724] = [
    b"CarFP\0" as *const u8 as *const libc::c_char,
    b"CarLP\0" as *const u8 as *const libc::c_char,
    b"CarSP\0" as *const u8 as *const libc::c_char,
    b"CarImmediate\0" as *const u8 as *const libc::c_char,
    b"CarPop\0" as *const u8 as *const libc::c_char,
    b"CdrFP\0" as *const u8 as *const libc::c_char,
    b"CdrLP\0" as *const u8 as *const libc::c_char,
    b"CdrSP\0" as *const u8 as *const libc::c_char,
    b"CdrImmediate\0" as *const u8 as *const libc::c_char,
    b"CdrPop\0" as *const u8 as *const libc::c_char,
    b"EndpFP\0" as *const u8 as *const libc::c_char,
    b"EndpLP\0" as *const u8 as *const libc::c_char,
    b"EndpSP\0" as *const u8 as *const libc::c_char,
    b"EndpImmediate\0" as *const u8 as *const libc::c_char,
    b"EndpPop\0" as *const u8 as *const libc::c_char,
    b"Setup1dArrayFP\0" as *const u8 as *const libc::c_char,
    b"Setup1dArrayLP\0" as *const u8 as *const libc::c_char,
    b"Setup1dArraySP\0" as *const u8 as *const libc::c_char,
    b"Setup1dArrayImmediate\0" as *const u8 as *const libc::c_char,
    b"Setup1dArrayPop\0" as *const u8 as *const libc::c_char,
    b"SetupForce1dArrayFP\0" as *const u8 as *const libc::c_char,
    b"SetupForce1dArrayLP\0" as *const u8 as *const libc::c_char,
    b"SetupForce1dArraySP\0" as *const u8 as *const libc::c_char,
    b"SetupForce1dArrayImmediate\0" as *const u8 as *const libc::c_char,
    b"SetupForce1dArrayPop\0" as *const u8 as *const libc::c_char,
    b"BindLocativeFP\0" as *const u8 as *const libc::c_char,
    b"BindLocativeLP\0" as *const u8 as *const libc::c_char,
    b"BindLocativeSP\0" as *const u8 as *const libc::c_char,
    b"BindLocativeImmediate\0" as *const u8 as *const libc::c_char,
    b"BindLocativePop\0" as *const u8 as *const libc::c_char,
    b"RestoreBindingStackFP\0" as *const u8 as *const libc::c_char,
    b"RestoreBindingStackLP\0" as *const u8 as *const libc::c_char,
    b"RestoreBindingStackSP\0" as *const u8 as *const libc::c_char,
    b"RestoreBindingStackImmediate\0" as *const u8 as *const libc::c_char,
    b"RestoreBindingStackPop\0" as *const u8 as *const libc::c_char,
    b"EphemeralpFP\0" as *const u8 as *const libc::c_char,
    b"EphemeralpLP\0" as *const u8 as *const libc::c_char,
    b"EphemeralpSP\0" as *const u8 as *const libc::c_char,
    b"EphemeralpImmediate\0" as *const u8 as *const libc::c_char,
    b"EphemeralpPop\0" as *const u8 as *const libc::c_char,
    b"StartCallFP\0" as *const u8 as *const libc::c_char,
    b"StartCallLP\0" as *const u8 as *const libc::c_char,
    b"StartCallSP\0" as *const u8 as *const libc::c_char,
    b"StartCallImmediate\0" as *const u8 as *const libc::c_char,
    b"StartCallPop\0" as *const u8 as *const libc::c_char,
    b"JumpFP\0" as *const u8 as *const libc::c_char,
    b"JumpLP\0" as *const u8 as *const libc::c_char,
    b"JumpSP\0" as *const u8 as *const libc::c_char,
    b"JumpImmediate\0" as *const u8 as *const libc::c_char,
    b"JumpPop\0" as *const u8 as *const libc::c_char,
    b"TagFP\0" as *const u8 as *const libc::c_char,
    b"TagLP\0" as *const u8 as *const libc::c_char,
    b"TagSP\0" as *const u8 as *const libc::c_char,
    b"TagImmediate\0" as *const u8 as *const libc::c_char,
    b"TagPop\0" as *const u8 as *const libc::c_char,
    b"DereferenceFP\0" as *const u8 as *const libc::c_char,
    b"DereferenceLP\0" as *const u8 as *const libc::c_char,
    b"DereferenceSP\0" as *const u8 as *const libc::c_char,
    b"DereferenceImmediate\0" as *const u8 as *const libc::c_char,
    b"DereferencePop\0" as *const u8 as *const libc::c_char,
    b"LogicTailTestFP\0" as *const u8 as *const libc::c_char,
    b"LogicTailTestLP\0" as *const u8 as *const libc::c_char,
    b"LogicTailTestSP\0" as *const u8 as *const libc::c_char,
    b"LogicTailTestImmediate\0" as *const u8 as *const libc::c_char,
    b"LogicTailTestPop\0" as *const u8 as *const libc::c_char,
    b"ProcBreakpointFP\0" as *const u8 as *const libc::c_char,
    b"ProcBreakpointLP\0" as *const u8 as *const libc::c_char,
    b"ProcBreakpointSP\0" as *const u8 as *const libc::c_char,
    b"ProcBreakpointImmediate\0" as *const u8 as *const libc::c_char,
    b"ProcBreakpointPop\0" as *const u8 as *const libc::c_char,
    b"PushLexicalVarFP\0" as *const u8 as *const libc::c_char,
    b"PushLexicalVarLP\0" as *const u8 as *const libc::c_char,
    b"PushLexicalVarSP\0" as *const u8 as *const libc::c_char,
    b"PushLexicalVarImmediate\0" as *const u8 as *const libc::c_char,
    b"PushLexicalVarPop\0" as *const u8 as *const libc::c_char,
    b"Block0WriteFP\0" as *const u8 as *const libc::c_char,
    b"Block0WriteLP\0" as *const u8 as *const libc::c_char,
    b"Block0WriteSP\0" as *const u8 as *const libc::c_char,
    b"Block0WriteImmediate\0" as *const u8 as *const libc::c_char,
    b"Block0WritePop\0" as *const u8 as *const libc::c_char,
    b"Block1WriteFP\0" as *const u8 as *const libc::c_char,
    b"Block1WriteLP\0" as *const u8 as *const libc::c_char,
    b"Block1WriteSP\0" as *const u8 as *const libc::c_char,
    b"Block1WriteImmediate\0" as *const u8 as *const libc::c_char,
    b"Block1WritePop\0" as *const u8 as *const libc::c_char,
    b"Block2WriteFP\0" as *const u8 as *const libc::c_char,
    b"Block2WriteLP\0" as *const u8 as *const libc::c_char,
    b"Block2WriteSP\0" as *const u8 as *const libc::c_char,
    b"Block2WriteImmediate\0" as *const u8 as *const libc::c_char,
    b"Block2WritePop\0" as *const u8 as *const libc::c_char,
    b"Block3WriteFP\0" as *const u8 as *const libc::c_char,
    b"Block3WriteLP\0" as *const u8 as *const libc::c_char,
    b"Block3WriteSP\0" as *const u8 as *const libc::c_char,
    b"Block3WriteImmediate\0" as *const u8 as *const libc::c_char,
    b"Block3WritePop\0" as *const u8 as *const libc::c_char,
    b"ZeropFP\0" as *const u8 as *const libc::c_char,
    b"ZeropLP\0" as *const u8 as *const libc::c_char,
    b"ZeropSP\0" as *const u8 as *const libc::c_char,
    b"ZeropImmediate\0" as *const u8 as *const libc::c_char,
    b"ZeropPop\0" as *const u8 as *const libc::c_char,
    b"MinuspFP\0" as *const u8 as *const libc::c_char,
    b"MinuspLP\0" as *const u8 as *const libc::c_char,
    b"MinuspSP\0" as *const u8 as *const libc::c_char,
    b"MinuspImmediate\0" as *const u8 as *const libc::c_char,
    b"MinuspPop\0" as *const u8 as *const libc::c_char,
    b"PluspFP\0" as *const u8 as *const libc::c_char,
    b"PluspLP\0" as *const u8 as *const libc::c_char,
    b"PluspSP\0" as *const u8 as *const libc::c_char,
    b"PluspImmediate\0" as *const u8 as *const libc::c_char,
    b"PluspPop\0" as *const u8 as *const libc::c_char,
    b"TypeMember\0" as *const u8 as *const libc::c_char,
    b"TypeMemberNoPop\0" as *const u8 as *const libc::c_char,
    b"LocateLocals\0" as *const u8 as *const libc::c_char,
    b"CatchClose\0" as *const u8 as *const libc::c_char,
    b"GenericDispatch\0" as *const u8 as *const libc::c_char,
    b"MessageDispatch\0" as *const u8 as *const libc::c_char,
    b"CheckPreemptRequest\0" as *const u8 as *const libc::c_char,
    b"PushGlobalLogicVariable\0" as *const u8 as *const libc::c_char,
    b"NoOp\0" as *const u8 as *const libc::c_char,
    b"Halt\0" as *const u8 as *const libc::c_char,
    b"BranchTrue\0" as *const u8 as *const libc::c_char,
    b"BranchTrueElseExtraPop\0" as *const u8 as *const libc::c_char,
    b"BranchTrueAndExtraPop\0" as *const u8 as *const libc::c_char,
    b"BranchTrueExtraPop\0" as *const u8 as *const libc::c_char,
    b"BranchTrueNoPop\0" as *const u8 as *const libc::c_char,
    b"BranchTrueAndNoPop\0" as *const u8 as *const libc::c_char,
    b"BranchTrueElseNoPop\0" as *const u8 as *const libc::c_char,
    b"BranchTrueAndNoPopElseNoPopExtraPop\0" as *const u8 as *const libc::c_char,
    b"BranchFalse\0" as *const u8 as *const libc::c_char,
    b"BranchFalseElseExtraPop\0" as *const u8 as *const libc::c_char,
    b"BranchFalseAndExtraPop\0" as *const u8 as *const libc::c_char,
    b"BranchFalseExtraPop\0" as *const u8 as *const libc::c_char,
    b"BranchFalseNoPop\0" as *const u8 as *const libc::c_char,
    b"BranchFalseAndNoPop\0" as *const u8 as *const libc::c_char,
    b"BranchFalseElseNoPop\0" as *const u8 as *const libc::c_char,
    b"BranchFalseAndNoPopElseNoPopExtraPop\0" as *const u8 as *const libc::c_char,
    b"PushFP\0" as *const u8 as *const libc::c_char,
    b"PushLP\0" as *const u8 as *const libc::c_char,
    b"PushSP\0" as *const u8 as *const libc::c_char,
    b"PushImmediate\0" as *const u8 as *const libc::c_char,
    b"PushPop\0" as *const u8 as *const libc::c_char,
    b"PushNNils\0" as *const u8 as *const libc::c_char,
    b"PushAddressSpRelativeFP\0" as *const u8 as *const libc::c_char,
    b"PushAddressSpRelativeLP\0" as *const u8 as *const libc::c_char,
    b"PushAddressSpRelativeSP\0" as *const u8 as *const libc::c_char,
    b"PushAddressSpRelativeImmediate\0" as *const u8 as *const libc::c_char,
    b"PushAddressSpRelativePop\0" as *const u8 as *const libc::c_char,
    b"PushLocalLogicVariablesFP\0" as *const u8 as *const libc::c_char,
    b"PushLocalLogicVariablesLP\0" as *const u8 as *const libc::c_char,
    b"PushLocalLogicVariablesSP\0" as *const u8 as *const libc::c_char,
    b"PushLocalLogicVariablesImmediate\0" as *const u8 as *const libc::c_char,
    b"PushLocalLogicVariablesPop\0" as *const u8 as *const libc::c_char,
    b"ReturnMultipleFP\0" as *const u8 as *const libc::c_char,
    b"ReturnMultipleLP\0" as *const u8 as *const libc::c_char,
    b"ReturnMultipleSP\0" as *const u8 as *const libc::c_char,
    b"ReturnMultipleImmediate\0" as *const u8 as *const libc::c_char,
    b"ReturnMultiplePop\0" as *const u8 as *const libc::c_char,
    b"ReturnKludgeFP\0" as *const u8 as *const libc::c_char,
    b"ReturnKludgeLP\0" as *const u8 as *const libc::c_char,
    b"ReturnKludgeSP\0" as *const u8 as *const libc::c_char,
    b"ReturnKludgeImmediate\0" as *const u8 as *const libc::c_char,
    b"ReturnKludgePop\0" as *const u8 as *const libc::c_char,
    b"TakeValues\0" as *const u8 as *const libc::c_char,
    b"UnbindNImmediate\0" as *const u8 as *const libc::c_char,
    b"UnbindNPop\0" as *const u8 as *const libc::c_char,
    b"PushInstanceVariable\0" as *const u8 as *const libc::c_char,
    b"PushAddressInstanceVariable\0" as *const u8 as *const libc::c_char,
    b"PushInstanceVariableOrdered\0" as *const u8 as *const libc::c_char,
    b"PushAddressInstanceVariableOrdered\0" as *const u8 as *const libc::c_char,
    b"UnaryMinusFP\0" as *const u8 as *const libc::c_char,
    b"UnaryMinusLP\0" as *const u8 as *const libc::c_char,
    b"UnaryMinusSP\0" as *const u8 as *const libc::c_char,
    b"UnaryMinusImmediate\0" as *const u8 as *const libc::c_char,
    b"UnaryMinusPop\0" as *const u8 as *const libc::c_char,
    b"ReturnSingleNIL\0" as *const u8 as *const libc::c_char,
    b"ReturnSingleT\0" as *const u8 as *const libc::c_char,
    b"ReturnSingleTOS\0" as *const u8 as *const libc::c_char,
    b"MemoryRead\0" as *const u8 as *const libc::c_char,
    b"MemoryReadAddress\0" as *const u8 as *const libc::c_char,
    b"Block0Read\0" as *const u8 as *const libc::c_char,
    b"Block1Read\0" as *const u8 as *const libc::c_char,
    b"Block2Read\0" as *const u8 as *const libc::c_char,
    b"Block3Read\0" as *const u8 as *const libc::c_char,
    b"Block0ReadShift\0" as *const u8 as *const libc::c_char,
    b"Block1ReadShift\0" as *const u8 as *const libc::c_char,
    b"Block2ReadShift\0" as *const u8 as *const libc::c_char,
    b"Block3ReadShift\0" as *const u8 as *const libc::c_char,
    b"Block0ReadTest\0" as *const u8 as *const libc::c_char,
    b"Block1ReadTest\0" as *const u8 as *const libc::c_char,
    b"Block2ReadTest\0" as *const u8 as *const libc::c_char,
    b"Block3ReadTest\0" as *const u8 as *const libc::c_char,
    b"FinishCallN\0" as *const u8 as *const libc::c_char,
    b"FinishCallNApply\0" as *const u8 as *const libc::c_char,
    b"FinishCallTos\0" as *const u8 as *const libc::c_char,
    b"FinishCallTosApply\0" as *const u8 as *const libc::c_char,
    b"SetToCarFP\0" as *const u8 as *const libc::c_char,
    b"SetToCarLP\0" as *const u8 as *const libc::c_char,
    b"SetToCarSP\0" as *const u8 as *const libc::c_char,
    b"SetToCarImmediate\0" as *const u8 as *const libc::c_char,
    b"SetToCarPop\0" as *const u8 as *const libc::c_char,
    b"SetToCdrFP\0" as *const u8 as *const libc::c_char,
    b"SetToCdrLP\0" as *const u8 as *const libc::c_char,
    b"SetToCdrSP\0" as *const u8 as *const libc::c_char,
    b"SetToCdrImmediate\0" as *const u8 as *const libc::c_char,
    b"SetToCdrPop\0" as *const u8 as *const libc::c_char,
    b"SetToCdrPushCarFP\0" as *const u8 as *const libc::c_char,
    b"SetToCdrPushCarLP\0" as *const u8 as *const libc::c_char,
    b"SetToCdrPushCarSP\0" as *const u8 as *const libc::c_char,
    b"SetToCdrPushCarImmediate\0" as *const u8 as *const libc::c_char,
    b"SetToCdrPushCarPop\0" as *const u8 as *const libc::c_char,
    b"IncrementFP\0" as *const u8 as *const libc::c_char,
    b"IncrementLP\0" as *const u8 as *const libc::c_char,
    b"IncrementSP\0" as *const u8 as *const libc::c_char,
    b"IncrementImmediate\0" as *const u8 as *const libc::c_char,
    b"IncrementPop\0" as *const u8 as *const libc::c_char,
    b"DecrementFP\0" as *const u8 as *const libc::c_char,
    b"DecrementLP\0" as *const u8 as *const libc::c_char,
    b"DecrementSP\0" as *const u8 as *const libc::c_char,
    b"DecrementImmediate\0" as *const u8 as *const libc::c_char,
    b"DecrementPop\0" as *const u8 as *const libc::c_char,
    b"PointerIncrementFP\0" as *const u8 as *const libc::c_char,
    b"PointerIncrementLP\0" as *const u8 as *const libc::c_char,
    b"PointerIncrementSP\0" as *const u8 as *const libc::c_char,
    b"PointerIncrementImmediate\0" as *const u8 as *const libc::c_char,
    b"PointerIncrementPop\0" as *const u8 as *const libc::c_char,
    b"SetCdrCode1FP\0" as *const u8 as *const libc::c_char,
    b"SetCdrCode1LP\0" as *const u8 as *const libc::c_char,
    b"SetCdrCode1SP\0" as *const u8 as *const libc::c_char,
    b"SetCdrCode1Immediate\0" as *const u8 as *const libc::c_char,
    b"SetCdrCode1Pop\0" as *const u8 as *const libc::c_char,
    b"SetCdrCode2FP\0" as *const u8 as *const libc::c_char,
    b"SetCdrCode2LP\0" as *const u8 as *const libc::c_char,
    b"SetCdrCode2SP\0" as *const u8 as *const libc::c_char,
    b"SetCdrCode2Immediate\0" as *const u8 as *const libc::c_char,
    b"SetCdrCode2Pop\0" as *const u8 as *const libc::c_char,
    b"PushAddressFP\0" as *const u8 as *const libc::c_char,
    b"PushAddressLP\0" as *const u8 as *const libc::c_char,
    b"PushAddressSP\0" as *const u8 as *const libc::c_char,
    b"PushAddressImmediate\0" as *const u8 as *const libc::c_char,
    b"PushAddressPop\0" as *const u8 as *const libc::c_char,
    b"SetSpToAddressFP\0" as *const u8 as *const libc::c_char,
    b"SetSpToAddressLP\0" as *const u8 as *const libc::c_char,
    b"SetSpToAddressSP\0" as *const u8 as *const libc::c_char,
    b"SetSpToAddressImmediate\0" as *const u8 as *const libc::c_char,
    b"SetSpToAddressPop\0" as *const u8 as *const libc::c_char,
    b"SetSpToAddressSaveTosFP\0" as *const u8 as *const libc::c_char,
    b"SetSpToAddressSaveTosLP\0" as *const u8 as *const libc::c_char,
    b"SetSpToAddressSaveTosSP\0" as *const u8 as *const libc::c_char,
    b"SetSpToAddressSaveTosImmediate\0" as *const u8 as *const libc::c_char,
    b"SetSpToAddressSaveTosPop\0" as *const u8 as *const libc::c_char,
    b"ReadInternalRegister\0" as *const u8 as *const libc::c_char,
    b"WriteInternalRegister\0" as *const u8 as *const libc::c_char,
    b"CoprocessorRead\0" as *const u8 as *const libc::c_char,
    b"CoprocessorWrite\0" as *const u8 as *const libc::c_char,
    b"Block0ReadAluFP\0" as *const u8 as *const libc::c_char,
    b"Block0ReadAluLP\0" as *const u8 as *const libc::c_char,
    b"Block0ReadAluSP\0" as *const u8 as *const libc::c_char,
    b"Block0ReadAluImmediate\0" as *const u8 as *const libc::c_char,
    b"Block0ReadAluPop\0" as *const u8 as *const libc::c_char,
    b"Block1ReadAluFP\0" as *const u8 as *const libc::c_char,
    b"Block1ReadAluLP\0" as *const u8 as *const libc::c_char,
    b"Block1ReadAluSP\0" as *const u8 as *const libc::c_char,
    b"Block1ReadAluImmediate\0" as *const u8 as *const libc::c_char,
    b"Block1ReadAluPop\0" as *const u8 as *const libc::c_char,
    b"Block2ReadAluFP\0" as *const u8 as *const libc::c_char,
    b"Block2ReadAluLP\0" as *const u8 as *const libc::c_char,
    b"Block2ReadAluSP\0" as *const u8 as *const libc::c_char,
    b"Block2ReadAluImmediate\0" as *const u8 as *const libc::c_char,
    b"Block2ReadAluPop\0" as *const u8 as *const libc::c_char,
    b"Block3ReadAluFP\0" as *const u8 as *const libc::c_char,
    b"Block3ReadAluLP\0" as *const u8 as *const libc::c_char,
    b"Block3ReadAluSP\0" as *const u8 as *const libc::c_char,
    b"Block3ReadAluImmediate\0" as *const u8 as *const libc::c_char,
    b"Block3ReadAluPop\0" as *const u8 as *const libc::c_char,
    b"Ldb\0" as *const u8 as *const libc::c_char,
    b"CharLdb\0" as *const u8 as *const libc::c_char,
    b"PLdb\0" as *const u8 as *const libc::c_char,
    b"PTagLdb\0" as *const u8 as *const libc::c_char,
    b"Branch\0" as *const u8 as *const libc::c_char,
    b"LoopDecrementTos\0" as *const u8 as *const libc::c_char,
    b"EntryRestAccepted\0" as *const u8 as *const libc::c_char,
    b"EntryRestNotAccepted\0" as *const u8 as *const libc::c_char,
    b"RplacaFP\0" as *const u8 as *const libc::c_char,
    b"RplacaLP\0" as *const u8 as *const libc::c_char,
    b"RplacaSP\0" as *const u8 as *const libc::c_char,
    b"RplacaImmediate\0" as *const u8 as *const libc::c_char,
    b"RplacaPop\0" as *const u8 as *const libc::c_char,
    b"RplacdFP\0" as *const u8 as *const libc::c_char,
    b"RplacdLP\0" as *const u8 as *const libc::c_char,
    b"RplacdSP\0" as *const u8 as *const libc::c_char,
    b"RplacdImmediate\0" as *const u8 as *const libc::c_char,
    b"RplacdPop\0" as *const u8 as *const libc::c_char,
    b"MultiplyFP\0" as *const u8 as *const libc::c_char,
    b"MultiplyLP\0" as *const u8 as *const libc::c_char,
    b"MultiplySP\0" as *const u8 as *const libc::c_char,
    b"MultiplyImmediate\0" as *const u8 as *const libc::c_char,
    b"MultiplyPop\0" as *const u8 as *const libc::c_char,
    b"QuotientFP\0" as *const u8 as *const libc::c_char,
    b"QuotientLP\0" as *const u8 as *const libc::c_char,
    b"QuotientSP\0" as *const u8 as *const libc::c_char,
    b"QuotientImmediate\0" as *const u8 as *const libc::c_char,
    b"QuotientPop\0" as *const u8 as *const libc::c_char,
    b"CeilingFP\0" as *const u8 as *const libc::c_char,
    b"CeilingLP\0" as *const u8 as *const libc::c_char,
    b"CeilingSP\0" as *const u8 as *const libc::c_char,
    b"CeilingImmediate\0" as *const u8 as *const libc::c_char,
    b"CeilingPop\0" as *const u8 as *const libc::c_char,
    b"FloorFP\0" as *const u8 as *const libc::c_char,
    b"FloorLP\0" as *const u8 as *const libc::c_char,
    b"FloorSP\0" as *const u8 as *const libc::c_char,
    b"FloorImmediate\0" as *const u8 as *const libc::c_char,
    b"FloorPop\0" as *const u8 as *const libc::c_char,
    b"TruncateFP\0" as *const u8 as *const libc::c_char,
    b"TruncateLP\0" as *const u8 as *const libc::c_char,
    b"TruncateSP\0" as *const u8 as *const libc::c_char,
    b"TruncateImmediate\0" as *const u8 as *const libc::c_char,
    b"TruncatePop\0" as *const u8 as *const libc::c_char,
    b"RoundFP\0" as *const u8 as *const libc::c_char,
    b"RoundLP\0" as *const u8 as *const libc::c_char,
    b"RoundSP\0" as *const u8 as *const libc::c_char,
    b"RoundImmediate\0" as *const u8 as *const libc::c_char,
    b"RoundPop\0" as *const u8 as *const libc::c_char,
    b"RationalQuotientFP\0" as *const u8 as *const libc::c_char,
    b"RationalQuotientLP\0" as *const u8 as *const libc::c_char,
    b"RationalQuotientSP\0" as *const u8 as *const libc::c_char,
    b"RationalQuotientImmediate\0" as *const u8 as *const libc::c_char,
    b"RationalQuotientPop\0" as *const u8 as *const libc::c_char,
    b"MinFP\0" as *const u8 as *const libc::c_char,
    b"MinLP\0" as *const u8 as *const libc::c_char,
    b"MinSP\0" as *const u8 as *const libc::c_char,
    b"MinImmediate\0" as *const u8 as *const libc::c_char,
    b"MinPop\0" as *const u8 as *const libc::c_char,
    b"MaxFP\0" as *const u8 as *const libc::c_char,
    b"MaxLP\0" as *const u8 as *const libc::c_char,
    b"MaxSP\0" as *const u8 as *const libc::c_char,
    b"MaxImmediate\0" as *const u8 as *const libc::c_char,
    b"MaxPop\0" as *const u8 as *const libc::c_char,
    b"AluFP\0" as *const u8 as *const libc::c_char,
    b"AluLP\0" as *const u8 as *const libc::c_char,
    b"AluSP\0" as *const u8 as *const libc::c_char,
    b"AluImmediate\0" as *const u8 as *const libc::c_char,
    b"AluPop\0" as *const u8 as *const libc::c_char,
    b"LogandFP\0" as *const u8 as *const libc::c_char,
    b"LogandLP\0" as *const u8 as *const libc::c_char,
    b"LogandSP\0" as *const u8 as *const libc::c_char,
    b"LogandImmediate\0" as *const u8 as *const libc::c_char,
    b"LogandPop\0" as *const u8 as *const libc::c_char,
    b"LogxorFP\0" as *const u8 as *const libc::c_char,
    b"LogxorLP\0" as *const u8 as *const libc::c_char,
    b"LogxorSP\0" as *const u8 as *const libc::c_char,
    b"LogxorImmediate\0" as *const u8 as *const libc::c_char,
    b"LogxorPop\0" as *const u8 as *const libc::c_char,
    b"LogiorFP\0" as *const u8 as *const libc::c_char,
    b"LogiorLP\0" as *const u8 as *const libc::c_char,
    b"LogiorSP\0" as *const u8 as *const libc::c_char,
    b"LogiorImmediate\0" as *const u8 as *const libc::c_char,
    b"LogiorPop\0" as *const u8 as *const libc::c_char,
    b"RotFP\0" as *const u8 as *const libc::c_char,
    b"RotLP\0" as *const u8 as *const libc::c_char,
    b"RotSP\0" as *const u8 as *const libc::c_char,
    b"RotImmediate\0" as *const u8 as *const libc::c_char,
    b"RotPop\0" as *const u8 as *const libc::c_char,
    b"LshFP\0" as *const u8 as *const libc::c_char,
    b"LshLP\0" as *const u8 as *const libc::c_char,
    b"LshSP\0" as *const u8 as *const libc::c_char,
    b"LshImmediate\0" as *const u8 as *const libc::c_char,
    b"LshPop\0" as *const u8 as *const libc::c_char,
    b"MultiplyDoubleFP\0" as *const u8 as *const libc::c_char,
    b"MultiplyDoubleLP\0" as *const u8 as *const libc::c_char,
    b"MultiplyDoubleSP\0" as *const u8 as *const libc::c_char,
    b"MultiplyDoubleImmediate\0" as *const u8 as *const libc::c_char,
    b"MultiplyDoublePop\0" as *const u8 as *const libc::c_char,
    b"LshcBignumStepFP\0" as *const u8 as *const libc::c_char,
    b"LshcBignumStepLP\0" as *const u8 as *const libc::c_char,
    b"LshcBignumStepSP\0" as *const u8 as *const libc::c_char,
    b"LshcBignumStepImmediate\0" as *const u8 as *const libc::c_char,
    b"LshcBignumStepPop\0" as *const u8 as *const libc::c_char,
    b"StackBltFP\0" as *const u8 as *const libc::c_char,
    b"StackBltLP\0" as *const u8 as *const libc::c_char,
    b"StackBltSP\0" as *const u8 as *const libc::c_char,
    b"StackBltImmediate\0" as *const u8 as *const libc::c_char,
    b"StackBltPop\0" as *const u8 as *const libc::c_char,
    b"RgetfFP\0" as *const u8 as *const libc::c_char,
    b"RgetfLP\0" as *const u8 as *const libc::c_char,
    b"RgetfSP\0" as *const u8 as *const libc::c_char,
    b"RgetfImmediate\0" as *const u8 as *const libc::c_char,
    b"RgetfPop\0" as *const u8 as *const libc::c_char,
    b"MemberFP\0" as *const u8 as *const libc::c_char,
    b"MemberLP\0" as *const u8 as *const libc::c_char,
    b"MemberSP\0" as *const u8 as *const libc::c_char,
    b"MemberImmediate\0" as *const u8 as *const libc::c_char,
    b"MemberPop\0" as *const u8 as *const libc::c_char,
    b"AssocFP\0" as *const u8 as *const libc::c_char,
    b"AssocLP\0" as *const u8 as *const libc::c_char,
    b"AssocSP\0" as *const u8 as *const libc::c_char,
    b"AssocImmediate\0" as *const u8 as *const libc::c_char,
    b"AssocPop\0" as *const u8 as *const libc::c_char,
    b"PointerPlusFP\0" as *const u8 as *const libc::c_char,
    b"PointerPlusLP\0" as *const u8 as *const libc::c_char,
    b"PointerPlusSP\0" as *const u8 as *const libc::c_char,
    b"PointerPlusImmediate\0" as *const u8 as *const libc::c_char,
    b"PointerPlusPop\0" as *const u8 as *const libc::c_char,
    b"PointerDifferenceFP\0" as *const u8 as *const libc::c_char,
    b"PointerDifferenceLP\0" as *const u8 as *const libc::c_char,
    b"PointerDifferenceSP\0" as *const u8 as *const libc::c_char,
    b"PointerDifferenceImmediate\0" as *const u8 as *const libc::c_char,
    b"PointerDifferencePop\0" as *const u8 as *const libc::c_char,
    b"AshFP\0" as *const u8 as *const libc::c_char,
    b"AshLP\0" as *const u8 as *const libc::c_char,
    b"AshSP\0" as *const u8 as *const libc::c_char,
    b"AshImmediate\0" as *const u8 as *const libc::c_char,
    b"AshPop\0" as *const u8 as *const libc::c_char,
    b"StoreConditionalFP\0" as *const u8 as *const libc::c_char,
    b"StoreConditionalLP\0" as *const u8 as *const libc::c_char,
    b"StoreConditionalSP\0" as *const u8 as *const libc::c_char,
    b"StoreConditionalImmediate\0" as *const u8 as *const libc::c_char,
    b"StoreConditionalPop\0" as *const u8 as *const libc::c_char,
    b"MemoryWriteFP\0" as *const u8 as *const libc::c_char,
    b"MemoryWriteLP\0" as *const u8 as *const libc::c_char,
    b"MemoryWriteSP\0" as *const u8 as *const libc::c_char,
    b"MemoryWriteImmediate\0" as *const u8 as *const libc::c_char,
    b"MemoryWritePop\0" as *const u8 as *const libc::c_char,
    b"PStoreContentsFP\0" as *const u8 as *const libc::c_char,
    b"PStoreContentsLP\0" as *const u8 as *const libc::c_char,
    b"PStoreContentsSP\0" as *const u8 as *const libc::c_char,
    b"PStoreContentsImmediate\0" as *const u8 as *const libc::c_char,
    b"PStoreContentsPop\0" as *const u8 as *const libc::c_char,
    b"BindLocativeToValueFP\0" as *const u8 as *const libc::c_char,
    b"BindLocativeToValueLP\0" as *const u8 as *const libc::c_char,
    b"BindLocativeToValueSP\0" as *const u8 as *const libc::c_char,
    b"BindLocativeToValueImmediate\0" as *const u8 as *const libc::c_char,
    b"BindLocativeToValuePop\0" as *const u8 as *const libc::c_char,
    b"UnifyFP\0" as *const u8 as *const libc::c_char,
    b"UnifyLP\0" as *const u8 as *const libc::c_char,
    b"UnifySP\0" as *const u8 as *const libc::c_char,
    b"UnifyImmediate\0" as *const u8 as *const libc::c_char,
    b"UnifyPop\0" as *const u8 as *const libc::c_char,
    b"PopLexicalVarFP\0" as *const u8 as *const libc::c_char,
    b"PopLexicalVarLP\0" as *const u8 as *const libc::c_char,
    b"PopLexicalVarSP\0" as *const u8 as *const libc::c_char,
    b"PopLexicalVarImmediate\0" as *const u8 as *const libc::c_char,
    b"PopLexicalVarPop\0" as *const u8 as *const libc::c_char,
    b"MovemLexicalVarFP\0" as *const u8 as *const libc::c_char,
    b"MovemLexicalVarLP\0" as *const u8 as *const libc::c_char,
    b"MovemLexicalVarSP\0" as *const u8 as *const libc::c_char,
    b"MovemLexicalVarImmediate\0" as *const u8 as *const libc::c_char,
    b"MovemLexicalVarPop\0" as *const u8 as *const libc::c_char,
    b"EqualNumberFP\0" as *const u8 as *const libc::c_char,
    b"EqualNumberLP\0" as *const u8 as *const libc::c_char,
    b"EqualNumberSP\0" as *const u8 as *const libc::c_char,
    b"EqualNumberImmediate\0" as *const u8 as *const libc::c_char,
    b"EqualNumberPop\0" as *const u8 as *const libc::c_char,
    b"LesspFP\0" as *const u8 as *const libc::c_char,
    b"LesspLP\0" as *const u8 as *const libc::c_char,
    b"LesspSP\0" as *const u8 as *const libc::c_char,
    b"LesspImmediate\0" as *const u8 as *const libc::c_char,
    b"LesspPop\0" as *const u8 as *const libc::c_char,
    b"GreaterpFP\0" as *const u8 as *const libc::c_char,
    b"GreaterpLP\0" as *const u8 as *const libc::c_char,
    b"GreaterpSP\0" as *const u8 as *const libc::c_char,
    b"GreaterpImmediate\0" as *const u8 as *const libc::c_char,
    b"GreaterpPop\0" as *const u8 as *const libc::c_char,
    b"EqlFP\0" as *const u8 as *const libc::c_char,
    b"EqlLP\0" as *const u8 as *const libc::c_char,
    b"EqlSP\0" as *const u8 as *const libc::c_char,
    b"EqlImmediate\0" as *const u8 as *const libc::c_char,
    b"EqlPop\0" as *const u8 as *const libc::c_char,
    b"EqualNumberNoPopFP\0" as *const u8 as *const libc::c_char,
    b"EqualNumberNoPopLP\0" as *const u8 as *const libc::c_char,
    b"EqualNumberNoPopSP\0" as *const u8 as *const libc::c_char,
    b"EqualNumberNoPopImmediate\0" as *const u8 as *const libc::c_char,
    b"EqualNumberNoPopPop\0" as *const u8 as *const libc::c_char,
    b"LesspNoPopFP\0" as *const u8 as *const libc::c_char,
    b"LesspNoPopLP\0" as *const u8 as *const libc::c_char,
    b"LesspNoPopSP\0" as *const u8 as *const libc::c_char,
    b"LesspNoPopImmediate\0" as *const u8 as *const libc::c_char,
    b"LesspNoPopPop\0" as *const u8 as *const libc::c_char,
    b"GreaterpNoPopFP\0" as *const u8 as *const libc::c_char,
    b"GreaterpNoPopLP\0" as *const u8 as *const libc::c_char,
    b"GreaterpNoPopSP\0" as *const u8 as *const libc::c_char,
    b"GreaterpNoPopImmediate\0" as *const u8 as *const libc::c_char,
    b"GreaterpNoPopPop\0" as *const u8 as *const libc::c_char,
    b"EqlNoPopFP\0" as *const u8 as *const libc::c_char,
    b"EqlNoPopLP\0" as *const u8 as *const libc::c_char,
    b"EqlNoPopSP\0" as *const u8 as *const libc::c_char,
    b"EqlNoPopImmediate\0" as *const u8 as *const libc::c_char,
    b"EqlNoPopPop\0" as *const u8 as *const libc::c_char,
    b"EqFP\0" as *const u8 as *const libc::c_char,
    b"EqLP\0" as *const u8 as *const libc::c_char,
    b"EqSP\0" as *const u8 as *const libc::c_char,
    b"EqImmediate\0" as *const u8 as *const libc::c_char,
    b"EqPop\0" as *const u8 as *const libc::c_char,
    b"LogtestFP\0" as *const u8 as *const libc::c_char,
    b"LogtestLP\0" as *const u8 as *const libc::c_char,
    b"LogtestSP\0" as *const u8 as *const libc::c_char,
    b"LogtestImmediate\0" as *const u8 as *const libc::c_char,
    b"LogtestPop\0" as *const u8 as *const libc::c_char,
    b"EqNoPopFP\0" as *const u8 as *const libc::c_char,
    b"EqNoPopLP\0" as *const u8 as *const libc::c_char,
    b"EqNoPopSP\0" as *const u8 as *const libc::c_char,
    b"EqNoPopImmediate\0" as *const u8 as *const libc::c_char,
    b"EqNoPopPop\0" as *const u8 as *const libc::c_char,
    b"LogtestNoPopFP\0" as *const u8 as *const libc::c_char,
    b"LogtestNoPopLP\0" as *const u8 as *const libc::c_char,
    b"LogtestNoPopSP\0" as *const u8 as *const libc::c_char,
    b"LogtestNoPopImmediate\0" as *const u8 as *const libc::c_char,
    b"LogtestNoPopPop\0" as *const u8 as *const libc::c_char,
    b"AddFP\0" as *const u8 as *const libc::c_char,
    b"AddLP\0" as *const u8 as *const libc::c_char,
    b"AddSP\0" as *const u8 as *const libc::c_char,
    b"AddImmediate\0" as *const u8 as *const libc::c_char,
    b"AddPop\0" as *const u8 as *const libc::c_char,
    b"SubFP\0" as *const u8 as *const libc::c_char,
    b"SubLP\0" as *const u8 as *const libc::c_char,
    b"SubSP\0" as *const u8 as *const libc::c_char,
    b"SubImmediate\0" as *const u8 as *const libc::c_char,
    b"SubPop\0" as *const u8 as *const libc::c_char,
    b"32BitPlusFP\0" as *const u8 as *const libc::c_char,
    b"32BitPlusLP\0" as *const u8 as *const libc::c_char,
    b"32BitPlusSP\0" as *const u8 as *const libc::c_char,
    b"32BitPlusImmediate\0" as *const u8 as *const libc::c_char,
    b"32BitPlusPop\0" as *const u8 as *const libc::c_char,
    b"32BitDifferenceFP\0" as *const u8 as *const libc::c_char,
    b"32BitDifferenceLP\0" as *const u8 as *const libc::c_char,
    b"32BitDifferenceSP\0" as *const u8 as *const libc::c_char,
    b"32BitDifferenceImmediate\0" as *const u8 as *const libc::c_char,
    b"32BitDifferencePop\0" as *const u8 as *const libc::c_char,
    b"AddBignumStepFP\0" as *const u8 as *const libc::c_char,
    b"AddBignumStepLP\0" as *const u8 as *const libc::c_char,
    b"AddBignumStepSP\0" as *const u8 as *const libc::c_char,
    b"AddBignumStepImmediate\0" as *const u8 as *const libc::c_char,
    b"AddBignumStepPop\0" as *const u8 as *const libc::c_char,
    b"SubBignumStepFP\0" as *const u8 as *const libc::c_char,
    b"SubBignumStepLP\0" as *const u8 as *const libc::c_char,
    b"SubBignumStepSP\0" as *const u8 as *const libc::c_char,
    b"SubBignumStepImmediate\0" as *const u8 as *const libc::c_char,
    b"SubBignumStepPop\0" as *const u8 as *const libc::c_char,
    b"MultiplyBignumStepFP\0" as *const u8 as *const libc::c_char,
    b"MultiplyBignumStepLP\0" as *const u8 as *const libc::c_char,
    b"MultiplyBignumStepSP\0" as *const u8 as *const libc::c_char,
    b"MultiplyBignumStepImmediate\0" as *const u8 as *const libc::c_char,
    b"MultiplyBignumStepPop\0" as *const u8 as *const libc::c_char,
    b"DivideBignumStepFP\0" as *const u8 as *const libc::c_char,
    b"DivideBignumStepLP\0" as *const u8 as *const libc::c_char,
    b"DivideBignumStepSP\0" as *const u8 as *const libc::c_char,
    b"DivideBignumStepImmediate\0" as *const u8 as *const libc::c_char,
    b"DivideBignumStepPop\0" as *const u8 as *const libc::c_char,
    b"Aset1FP\0" as *const u8 as *const libc::c_char,
    b"Aset1LP\0" as *const u8 as *const libc::c_char,
    b"Aset1SP\0" as *const u8 as *const libc::c_char,
    b"Aset1Immediate\0" as *const u8 as *const libc::c_char,
    b"Aset1Pop\0" as *const u8 as *const libc::c_char,
    b"AllocateListBlockFP\0" as *const u8 as *const libc::c_char,
    b"AllocateListBlockLP\0" as *const u8 as *const libc::c_char,
    b"AllocateListBlockSP\0" as *const u8 as *const libc::c_char,
    b"AllocateListBlockImmediate\0" as *const u8 as *const libc::c_char,
    b"AllocateListBlockPop\0" as *const u8 as *const libc::c_char,
    b"Aref1FP\0" as *const u8 as *const libc::c_char,
    b"Aref1LP\0" as *const u8 as *const libc::c_char,
    b"Aref1SP\0" as *const u8 as *const libc::c_char,
    b"Aref1Immediate\0" as *const u8 as *const libc::c_char,
    b"Aref1Pop\0" as *const u8 as *const libc::c_char,
    b"Aloc1FP\0" as *const u8 as *const libc::c_char,
    b"Aloc1LP\0" as *const u8 as *const libc::c_char,
    b"Aloc1SP\0" as *const u8 as *const libc::c_char,
    b"Aloc1Immediate\0" as *const u8 as *const libc::c_char,
    b"Aloc1Pop\0" as *const u8 as *const libc::c_char,
    b"StoreArrayLeaderFP\0" as *const u8 as *const libc::c_char,
    b"StoreArrayLeaderLP\0" as *const u8 as *const libc::c_char,
    b"StoreArrayLeaderSP\0" as *const u8 as *const libc::c_char,
    b"StoreArrayLeaderImmediate\0" as *const u8 as *const libc::c_char,
    b"StoreArrayLeaderPop\0" as *const u8 as *const libc::c_char,
    b"AllocateStructureBlockFP\0" as *const u8 as *const libc::c_char,
    b"AllocateStructureBlockLP\0" as *const u8 as *const libc::c_char,
    b"AllocateStructureBlockSP\0" as *const u8 as *const libc::c_char,
    b"AllocateStructureBlockImmediate\0" as *const u8 as *const libc::c_char,
    b"AllocateStructureBlockPop\0" as *const u8 as *const libc::c_char,
    b"ArrayLeaderFP\0" as *const u8 as *const libc::c_char,
    b"ArrayLeaderLP\0" as *const u8 as *const libc::c_char,
    b"ArrayLeaderSP\0" as *const u8 as *const libc::c_char,
    b"ArrayLeaderImmediate\0" as *const u8 as *const libc::c_char,
    b"ArrayLeaderPop\0" as *const u8 as *const libc::c_char,
    b"AlocLeaderFP\0" as *const u8 as *const libc::c_char,
    b"AlocLeaderLP\0" as *const u8 as *const libc::c_char,
    b"AlocLeaderSP\0" as *const u8 as *const libc::c_char,
    b"AlocLeaderImmediate\0" as *const u8 as *const libc::c_char,
    b"AlocLeaderPop\0" as *const u8 as *const libc::c_char,
    b"PopInstanceVariable\0" as *const u8 as *const libc::c_char,
    b"MovemInstanceVariable\0" as *const u8 as *const libc::c_char,
    b"PopInstanceVariableOrdered\0" as *const u8 as *const libc::c_char,
    b"MovemInstanceVariableOrdered\0" as *const u8 as *const libc::c_char,
    b"InstanceRefFP\0" as *const u8 as *const libc::c_char,
    b"InstanceRefLP\0" as *const u8 as *const libc::c_char,
    b"InstanceRefSP\0" as *const u8 as *const libc::c_char,
    b"InstanceRefImmediate\0" as *const u8 as *const libc::c_char,
    b"InstanceRefPop\0" as *const u8 as *const libc::c_char,
    b"InstanceSetFP\0" as *const u8 as *const libc::c_char,
    b"InstanceSetLP\0" as *const u8 as *const libc::c_char,
    b"InstanceSetSP\0" as *const u8 as *const libc::c_char,
    b"InstanceSetImmediate\0" as *const u8 as *const libc::c_char,
    b"InstanceSetPop\0" as *const u8 as *const libc::c_char,
    b"InstanceLocFP\0" as *const u8 as *const libc::c_char,
    b"InstanceLocLP\0" as *const u8 as *const libc::c_char,
    b"InstanceLocSP\0" as *const u8 as *const libc::c_char,
    b"InstanceLocImmediate\0" as *const u8 as *const libc::c_char,
    b"InstanceLocPop\0" as *const u8 as *const libc::c_char,
    b"SetTagFP\0" as *const u8 as *const libc::c_char,
    b"SetTagLP\0" as *const u8 as *const libc::c_char,
    b"SetTagSP\0" as *const u8 as *const libc::c_char,
    b"SetTagImmediate\0" as *const u8 as *const libc::c_char,
    b"SetTagPop\0" as *const u8 as *const libc::c_char,
    b"UnsignedLesspFP\0" as *const u8 as *const libc::c_char,
    b"UnsignedLesspLP\0" as *const u8 as *const libc::c_char,
    b"UnsignedLesspSP\0" as *const u8 as *const libc::c_char,
    b"UnsignedLesspImmediate\0" as *const u8 as *const libc::c_char,
    b"UnsignedLesspPop\0" as *const u8 as *const libc::c_char,
    b"UnsignedLesspNoPopFP\0" as *const u8 as *const libc::c_char,
    b"UnsignedLesspNoPopLP\0" as *const u8 as *const libc::c_char,
    b"UnsignedLesspNoPopSP\0" as *const u8 as *const libc::c_char,
    b"UnsignedLesspNoPopImmediate\0" as *const u8 as *const libc::c_char,
    b"UnsignedLesspNoPopPop\0" as *const u8 as *const libc::c_char,
    b"PopFP\0" as *const u8 as *const libc::c_char,
    b"PopLP\0" as *const u8 as *const libc::c_char,
    b"PopSP\0" as *const u8 as *const libc::c_char,
    b"PopImmediate\0" as *const u8 as *const libc::c_char,
    b"PopPop\0" as *const u8 as *const libc::c_char,
    b"MovemFP\0" as *const u8 as *const libc::c_char,
    b"MovemLP\0" as *const u8 as *const libc::c_char,
    b"MovemSP\0" as *const u8 as *const libc::c_char,
    b"MovemImmediate\0" as *const u8 as *const libc::c_char,
    b"MovemPop\0" as *const u8 as *const libc::c_char,
    b"MergeCdrNoPopFP\0" as *const u8 as *const libc::c_char,
    b"MergeCdrNoPopLP\0" as *const u8 as *const libc::c_char,
    b"MergeCdrNoPopSP\0" as *const u8 as *const libc::c_char,
    b"MergeCdrNoPopImmediate\0" as *const u8 as *const libc::c_char,
    b"MergeCdrNoPopPop\0" as *const u8 as *const libc::c_char,
    b"FastAref1FP\0" as *const u8 as *const libc::c_char,
    b"FastAref1LP\0" as *const u8 as *const libc::c_char,
    b"FastAref1SP\0" as *const u8 as *const libc::c_char,
    b"FastAref1Immediate\0" as *const u8 as *const libc::c_char,
    b"FastAref1Pop\0" as *const u8 as *const libc::c_char,
    b"FastAset1FP\0" as *const u8 as *const libc::c_char,
    b"FastAset1LP\0" as *const u8 as *const libc::c_char,
    b"FastAset1SP\0" as *const u8 as *const libc::c_char,
    b"FastAset1Immediate\0" as *const u8 as *const libc::c_char,
    b"FastAset1Pop\0" as *const u8 as *const libc::c_char,
    b"StackBltAddressFP\0" as *const u8 as *const libc::c_char,
    b"StackBltAddressLP\0" as *const u8 as *const libc::c_char,
    b"StackBltAddressSP\0" as *const u8 as *const libc::c_char,
    b"StackBltAddressImmediate\0" as *const u8 as *const libc::c_char,
    b"StackBltAddressPop\0" as *const u8 as *const libc::c_char,
    b"Dpb\0" as *const u8 as *const libc::c_char,
    b"CharDpb\0" as *const u8 as *const libc::c_char,
    b"PDpb\0" as *const u8 as *const libc::c_char,
    b"PTagDpb\0" as *const u8 as *const libc::c_char,
    b"LoopIncrementTosLessThan\0" as *const u8 as *const libc::c_char,
    b"CatchOpen\0" as *const u8 as *const libc::c_char,
    b"Hack\0" as *const u8 as *const libc::c_char,
    b"PushNull\0" as *const u8 as *const libc::c_char,
    b"PushMonitorForward\0" as *const u8 as *const libc::c_char,
    b"PushHeaderP\0" as *const u8 as *const libc::c_char,
    b"PushHeaderI\0" as *const u8 as *const libc::c_char,
    b"PushExternalValueCellPointer\0" as *const u8 as *const libc::c_char,
    b"PushOneQForward\0" as *const u8 as *const libc::c_char,
    b"PushHeaderForward\0" as *const u8 as *const libc::c_char,
    b"PushElementForward\0" as *const u8 as *const libc::c_char,
    b"PushFixnum\0" as *const u8 as *const libc::c_char,
    b"PushSmallRatio\0" as *const u8 as *const libc::c_char,
    b"PushSingleFloat\0" as *const u8 as *const libc::c_char,
    b"PushDoubleFloat\0" as *const u8 as *const libc::c_char,
    b"PushBignum\0" as *const u8 as *const libc::c_char,
    b"PushBigRatio\0" as *const u8 as *const libc::c_char,
    b"PushComplex\0" as *const u8 as *const libc::c_char,
    b"PushSpareNumber\0" as *const u8 as *const libc::c_char,
    b"PushInstance\0" as *const u8 as *const libc::c_char,
    b"PushListInstance\0" as *const u8 as *const libc::c_char,
    b"PushArrayInstance\0" as *const u8 as *const libc::c_char,
    b"PushStringInstance\0" as *const u8 as *const libc::c_char,
    b"PushNil\0" as *const u8 as *const libc::c_char,
    b"PushList\0" as *const u8 as *const libc::c_char,
    b"PushArray\0" as *const u8 as *const libc::c_char,
    b"PushString\0" as *const u8 as *const libc::c_char,
    b"PushSymbol\0" as *const u8 as *const libc::c_char,
    b"PushLocative\0" as *const u8 as *const libc::c_char,
    b"PushLexicalClosure\0" as *const u8 as *const libc::c_char,
    b"PushDynamicClosure\0" as *const u8 as *const libc::c_char,
    b"PushCompiledFunction\0" as *const u8 as *const libc::c_char,
    b"PushGenericFunction\0" as *const u8 as *const libc::c_char,
    b"PushSparePointer1\0" as *const u8 as *const libc::c_char,
    b"PushSparePointer2\0" as *const u8 as *const libc::c_char,
    b"PushPhysicalAddress\0" as *const u8 as *const libc::c_char,
    b"PushSpareImmediate1\0" as *const u8 as *const libc::c_char,
    b"PushBoundLocation\0" as *const u8 as *const libc::c_char,
    b"PushCharacter\0" as *const u8 as *const libc::c_char,
    b"PushLogicVariable\0" as *const u8 as *const libc::c_char,
    b"PushGcForward\0" as *const u8 as *const libc::c_char,
    b"PushEvenPc\0" as *const u8 as *const libc::c_char,
    b"PushOddPc\0" as *const u8 as *const libc::c_char,
    b"CallCompiledEven\0" as *const u8 as *const libc::c_char,
    b"CallCompiledOdd\0" as *const u8 as *const libc::c_char,
    b"CallIndirect\0" as *const u8 as *const libc::c_char,
    b"CallGeneric\0" as *const u8 as *const libc::c_char,
    b"CallCompiledEvenPrefetch\0" as *const u8 as *const libc::c_char,
    b"CallCompiledOddPrefetch\0" as *const u8 as *const libc::c_char,
    b"CallIndirectPrefetch\0" as *const u8 as *const libc::c_char,
    b"CallGenericPrefetch\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction60\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction61\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction62\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction63\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction64\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction65\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction66\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction67\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction70\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction71\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction72\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction73\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction74\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction75\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction76\0" as *const u8 as *const libc::c_char,
    b"PushPackedInstruction77\0" as *const u8 as *const libc::c_char,
    b"InstructionCacheLookup\0" as *const u8 as *const libc::c_char,
    b"IllegalInstruction\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub  fn InstructionCacheMiss() -> usize {
    let mut block_vma: isize = 0;
    let mut block_cp: *mut InstructionCacheLine = 0 as *mut InstructionCacheLine;
    let mut block_instruction: *mut LispObj = 0 as *mut LispObj;
    block_vma = ((*processor).pc.parts.data.u
        & !((64 as usize >> 1) - 1) as libc::c_uint)
        as isize;
    block_cp = ((*processor).InstructionCache)
        .offset(
            (block_vma << 1
                & (2048 as usize - 1) as libc::c_ulong) as isize,
        );
    let mut bound_vma: isize = block_vma
        .wrapping_add((64 as usize >> 1) as libc::c_ulong);
    let mut vma: isize = 0;
    let mut even_cp: *mut InstructionCacheLine = 0 as *mut InstructionCacheLine;
    let mut odd_cp: *mut InstructionCacheLine = 0 as *mut InstructionCacheLine;
    let mut instruction_block: [LispObj; 32] = [LispObj {
        parts: _LispObj {
            tag: 0,
            data: C2RustUnnamed { u: 0 },
        },
    }; 32];
    let mut instruction: *mut LispObj = &mut *instruction_block
        .as_mut_ptr()
        .offset(0 as usize as isize) as *mut LispObj;
    let mut tag: Tag = 0;
    let mut data: isize = 0;
    VirtualMemoryReadBlock(
        block_vma,
        instruction_block.as_mut_ptr(),
        64 as usize >> 1,
    );
    vma = block_vma;
    even_cp = block_cp;
    odd_cp = block_cp.offset(1 as usize as isize);
    while vma < bound_vma {
        let mut current_block_50: u64;
        tag = (*instruction).parts.tag as Tag;
        data = (*instruction).parts.data.u as isize;
        (*even_cp).pc.parts.data.u = vma as ui32;
        (*odd_cp).pc.parts.data.u = vma as ui32;
        match tag as usize >> 6 as usize {
            0 => {
                (*even_cp).next_pc.parts.tag = TypeOddPC as usize as ui32;
                (*even_cp).next_pc.parts.data.u = vma as ui32;
                let ref mut fresh0 = (*even_cp).next_cp;
                *fresh0 = odd_cp;
                (*odd_cp).next_pc.parts.tag = TypeEvenPC as usize as ui32;
                (*odd_cp)
                    .next_pc
                    .parts
                    .data
                    .u = vma.wrapping_add(1 as usize as libc::c_ulong) as ui32;
                let ref mut fresh1 = (*odd_cp).next_cp;
                *fresh1 = even_cp.offset(2 as usize as isize);
                current_block_50 = 17506352804191194145;
            }
            3 => {
                (*even_cp).next_pc.parts.tag = TypeEvenPC as usize as ui32;
                (*even_cp)
                    .next_pc
                    .parts
                    .data
                    .u = vma.wrapping_add(1 as usize as libc::c_ulong) as ui32;
                let ref mut fresh2 = (*even_cp).next_cp;
                *fresh2 = even_cp.offset(2 as usize as isize);
                (*odd_cp).next_pc.parts.tag = TypeEvenPC as usize as ui32;
                (*odd_cp)
                    .next_pc
                    .parts
                    .data
                    .u = vma.wrapping_add(2 as usize as libc::c_ulong) as ui32;
                let ref mut fresh3 = (*odd_cp).next_cp;
                *fresh3 = even_cp.offset(4 as usize as isize);
                current_block_50 = 15597372965620363352;
            }
            2 => {
                (*even_cp).next_pc.parts.tag = TypeOddPC as usize as ui32;
                (*even_cp)
                    .next_pc
                    .parts
                    .data
                    .u = vma.wrapping_sub(1 as usize as libc::c_ulong) as ui32;
                let ref mut fresh4 = (*even_cp).next_cp;
                *fresh4 = odd_cp.offset(-(2 as usize as isize));
                (*odd_cp).next_pc.parts.tag = TypeEvenPC as usize as ui32;
                (*odd_cp).next_pc.parts.data.u = vma as ui32;
                let ref mut fresh5 = (*odd_cp).next_cp;
                *fresh5 = &mut InstructionCacheLookupCPRepresentation;
                current_block_50 = 15597372965620363352;
            }
            1 => {
                (*even_cp).code = IllegalInstructionDecoder;
                (*even_cp).operand = 0;
                (*even_cp).instruction = 0 as usize as libc::c_uint;
                (*odd_cp).code = IllegalInstructionDecoder;
                (*odd_cp).operand = 0;
                (*odd_cp).instruction = 0 as usize as libc::c_uint;
                current_block_50 = 6072622540298447352;
            }
            _ => {
                current_block_50 = 15597372965620363352;
            }
        }
        match current_block_50 {
            15597372965620363352 => {
                if !(tag as usize & 0o60 as usize == 0o60) {
                    (*even_cp)
                        .code = WordInstructionDecoder[(tag
                        & ((1) << 6) - 1)
                        as usize];
                    (*even_cp).operand = data;
                    (*even_cp).instruction = 0 as usize as libc::c_uint;
                    (*odd_cp).code = IllegalInstructionDecoder;
                    (*odd_cp).operand = 0;
                    (*odd_cp).instruction = 0 as usize as libc::c_uint;
                    current_block_50 = 6072622540298447352;
                } else {
                    current_block_50 = 17506352804191194145;
                }
            }
            _ => {}
        }
        match current_block_50 {
            17506352804191194145 => {
                let mut instruction_0: isize = 0;
                let mut p: *const DecoderPair = 0 as *const DecoderPair;
                instruction_0 = data & 0o777777 as usize as libc::c_ulong;
                (*even_cp).instruction = instruction_0 as libc::c_uint;
                p = PackedInstructionDecoder
                    .as_ptr()
                    .offset((instruction_0 >> 8) as isize);
                (*even_cp).code = (*p).dispatch;
                ::std::mem::transmute::<
                    _,
                    fn(_, _),
                >(((*p).decode).expect("non-null function pointer"))(data, even_cp);
                instruction_0 = ((tag as usize & 0o17)
                    << 14
                    | (data >> 18
                        & (((1) << 14) - 1)
                            as libc::c_ulong)) as isize;
                (*odd_cp).instruction = instruction_0 as libc::c_uint;
                p = PackedInstructionDecoder
                    .as_ptr()
                    .offset((instruction_0 >> 8) as isize);
                (*odd_cp).code = (*p).dispatch;
                ::std::mem::transmute::<
                    _,
                    fn(_, _),
                >(
                    ((*p).decode).expect("non-null function pointer"),
                )(instruction_0, odd_cp);
            }
            _ => {}
        }
        vma = vma.wrapping_add(1);
        instruction = instruction.offset(1);
        even_cp = even_cp.offset(2 as usize as isize);
        odd_cp = odd_cp.offset(2 as usize as isize);
    }
    if ((*block_cp).next_pc.parts.data.u as libc::c_ulong) < block_vma {
        let ref mut fresh6 = (*block_cp).next_cp;
        *fresh6 = &mut InstructionCacheLookupCPRepresentation;
    }
    even_cp = block_cp
        .offset(64 as usize as isize)
        .offset(-(4 as usize as isize));
    if (*even_cp).next_pc.parts.data.u as libc::c_ulong >= bound_vma {
        let ref mut fresh7 = (*even_cp).next_cp;
        *fresh7 = &mut InstructionCacheLookupCPRepresentation;
    }
    even_cp = even_cp.offset(1);
    if (*even_cp).next_pc.parts.data.u as libc::c_ulong >= bound_vma {
        let ref mut fresh8 = (*even_cp).next_cp;
        *fresh8 = &mut InstructionCacheLookupCPRepresentation;
    }
    even_cp = even_cp.offset(1);
    if (*even_cp).next_pc.parts.data.u as libc::c_ulong >= bound_vma {
        let ref mut fresh9 = (*even_cp).next_cp;
        *fresh9 = &mut InstructionCacheLookupCPRepresentation;
    }
    even_cp = even_cp.offset(1);
    if (*even_cp).next_pc.parts.data.u as libc::c_ulong >= bound_vma {
        let ref mut fresh10 = (*even_cp).next_cp;
        *fresh10 = &mut InstructionCacheLookupCPRepresentation;
    }
    return 0;
}
