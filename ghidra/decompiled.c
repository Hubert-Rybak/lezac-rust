// Decompiled from LEZAC.EXE using Ghidra
// Larax & Zaco v1.0 (1996) by Zanobi Software

// ================================================
// Function: FUN_1000_0000 at 1000:0000
// ================================================

void FUN_1000_0000(uint param_1,int *param_2,byte *param_3,undefined2 param_4)

{
  uint local_4;
  
  FUN_1920_04df();
  if (param_1 != 0) {
    local_4 = 1;
    while( true ) {
      if ((local_4 & 1) == 1) {
        *param_2 = *param_2 + (uint)*param_3;
      }
      else {
        *param_2 = *param_2 - (uint)*param_3;
      }
      param_3 = param_3 + 1;
      if (local_4 == param_1) break;
      local_4 = local_4 + 1;
    }
  }
  return;
}



// ================================================
// Function: FUN_1000_00a3 at 1000:00a3
// ================================================

void FUN_1000_00a3(char param_1)

{
  code *pcVar1;
  undefined2 unaff_DS;
  undefined2 uVar2;
  
  FUN_1920_04df();
  FUN_183f_0093(*(undefined2 *)0x1b60,*(undefined2 *)0x1b62,9);
  FUN_183f_0093(*(undefined2 *)0x1b64,*(undefined2 *)0x1b66,0x1c);
  FUN_184a_02f6();
  out(0x60,0xf3);
  FUN_184a_029c(200);
  out(0x60,0x47);
  pcVar1 = (code *)swi(0x10);
  (*pcVar1)();
  if (param_1 == '\x01') {
    uVar2 = 0xc59e;
    FUN_1920_08d0(0,0x68,0x184a);
    FUN_1920_084a(uVar2);
    FUN_1920_04a9();
  }
  else if (param_1 == '\x02') {
    uVar2 = 0xc59e;
    FUN_1920_08d0(0,0x90,0x184a);
    FUN_1920_084a(uVar2);
    FUN_1920_04a9();
  }
  *(undefined1 *)0x7f5c = *(undefined1 *)0x79c9;
  FUN_1920_00e9();
  return;
}



// ================================================
// Function: FUN_1000_0139 at 1000:0139
// ================================================

void FUN_1000_0139(void)

{
  char cVar1;
  char cVar2;
  char cVar3;
  char cVar4;
  char cVar5;
  char cVar6;
  byte local_3;
  
  FUN_1920_04df();
  cVar1 = FUN_1920_13a8(0x14);
  cVar2 = FUN_1920_13a8(0x14);
  cVar3 = FUN_1920_13a8(0x14);
  cVar4 = FUN_1920_13a8(0x1e);
  cVar5 = FUN_1920_13a8(0x1e);
  cVar6 = FUN_1920_13a8(0x1e);
  local_3 = 0xb0;
  while( true ) {
    FUN_18ac_0000((int)cVar3 + (int)((int)cVar6 * (local_3 - 0xb0)) / 7,
                  (int)cVar2 + (int)((int)cVar5 * (local_3 - 0xb0)) / 7,
                  (int)cVar1 + (int)((int)cVar4 * (local_3 - 0xb0)) / 7,local_3);
    if (local_3 == 0xb6) break;
    local_3 = local_3 + 1;
  }
  return;
}



// ================================================
// Function: FUN_1000_01fc at 1000:01fc
// ================================================

void __cdecl16near FUN_1000_01fc(void)

{
  int iVar1;
  int iVar2;
  int iVar3;
  uint local_10;
  char *local_e;
  uint local_c;
  int local_4;
  
  FUN_1920_04df();
  FUN_1000_07c5(0,200,0x140,0,0);
  local_e = (char *)0x0;
  local_10 = 0;
  local_4 = 0;
  iVar1 = FUN_1920_13a8(0x50);
  iVar2 = FUN_1920_13a8(0x50);
  FUN_1000_0139(&stack0xfffe);
  local_c = 0;
  while( true ) {
    local_4 = local_4 + iVar1 + 1;
    if (100 < local_4) {
      local_4 = local_4 + -100;
      local_10 = local_10 + 1;
    }
    if (local_c % 0x140 == 0) {
      FUN_1920_0f0f();
      FUN_1920_0efb();
      FUN_1920_0f13();
      FUN_1920_0945();
      FUN_1920_0efb();
      iVar3 = FUN_1920_0f13();
      local_4 = local_4 + iVar2 + 1 + iVar3;
      if (100 < local_4) {
        local_4 = local_4 + -100;
        local_10 = local_10 + 1;
      }
    }
    *local_e = (char)((ulong)local_10 % 7) + -0x50;
    local_e = local_e + 1;
    if (local_c == 63999) break;
    local_c = local_c + 1;
  }
  return;
}



// ================================================
// Function: FUN_1000_02d1 at 1000:02d1
// ================================================

void __cdecl16near FUN_1000_02d1(void)

{
  int iVar1;
  int in_AX;
  int unaff_BP;
  undefined2 unaff_SS;
  
  do {
    *(int *)(unaff_BP + -2) = *(int *)(unaff_BP + -2) + in_AX;
    if (100 < *(int *)(unaff_BP + -2)) {
      *(int *)(unaff_BP + -2) = *(int *)(unaff_BP + -2) + -100;
      *(int *)(unaff_BP + -0xe) = *(int *)(unaff_BP + -0xe) + 1;
    }
    do {
      *(char *)*(undefined2 *)(unaff_BP + -0xc) =
           (char)((ulong)*(uint *)(unaff_BP + -0xe) % 7) + -0x50;
      *(int *)(unaff_BP + -0xc) = *(int *)(unaff_BP + -0xc) + 1;
      if (*(int *)(unaff_BP + -10) == -0x601) {
        return;
      }
      *(int *)(unaff_BP + -10) = *(int *)(unaff_BP + -10) + 1;
      *(int *)(unaff_BP + -2) = *(int *)(unaff_BP + -2) + *(int *)(unaff_BP + -6);
      if (100 < *(int *)(unaff_BP + -2)) {
        *(int *)(unaff_BP + -2) = *(int *)(unaff_BP + -2) + -100;
        *(int *)(unaff_BP + -0xe) = *(int *)(unaff_BP + -0xe) + 1;
      }
    } while (*(uint *)(unaff_BP + -10) % 0x140 != 0);
    FUN_1920_0f0f();
    FUN_1920_0efb();
    FUN_1920_0f13();
    FUN_1920_0945();
    FUN_1920_0efb();
    iVar1 = FUN_1920_0f13();
    in_AX = *(int *)(unaff_BP + -8) + iVar1;
  } while( true );
}



// ================================================
// Function: FUN_1000_030b at 1000:030b
// ================================================

void FUN_1000_030b(undefined4 param_1)

{
  code *pcVar1;
  char cVar2;
  undefined1 uVar3;
  int iVar4;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  int local_798;
  int local_796;
  undefined2 local_794;
  undefined2 uStack_792;
  byte *local_790;
  byte local_78c [770];
  byte local_48a [770];
  undefined2 local_188;
  undefined2 local_186;
  undefined2 local_184;
  undefined1 local_182 [128];
  undefined1 local_102 [252];
  undefined2 uStack_6;
  undefined2 uStack_4;
  
  uStack_4 = 0x1000;
  uStack_6 = 0x316;
  FUN_1920_04df();
  FUN_1920_09f4(0xff,local_102,unaff_SS,(int)param_1,(int)((ulong)param_1 >> 0x10));
  FUN_1920_15ca(local_102,unaff_SS,local_182,unaff_SS);
  FUN_1920_15f8(1,local_182,unaff_SS);
  FUN_1920_16e3(0,0,0x300,local_48a,unaff_SS,local_182,unaff_SS);
  local_794 = *(undefined2 *)0x7f70;
  uStack_792 = *(undefined2 *)0x7f72;
  FUN_1920_16e3(0,0,2,&local_188,unaff_SS,local_182,unaff_SS);
  FUN_1920_16e3(0,0,local_188,local_794,uStack_792,local_182,unaff_SS);
  FUN_1920_1679(local_182,unaff_SS);
  iVar4 = FUN_1920_04a2();
  if (iVar4 != 0) {
    FUN_1000_00a3(1);
  }
  local_186 = 0;
  local_184 = 0xa000;
  local_796 = 0;
  while( true ) {
    cVar2 = FUN_184a_02fd();
    if ((cVar2 != '\0') && (2 < local_796)) {
      uVar3 = FUN_184a_030f();
      *(undefined1 *)0x2058 = uVar3;
      local_796 = 0x3f;
    }
    for (local_798 = 0;
        local_78c[local_798] = (byte)((int)((uint)local_48a[local_798] * local_796) / 0x3f),
        local_798 != 0x300; local_798 = local_798 + 1) {
    }
    for (local_798 = 0x1e; local_78c[local_798] = local_48a[local_798], local_798 != 0x20;
        local_798 = local_798 + 1) {
    }
    local_790 = local_78c;
    pcVar1 = (code *)swi(0x10);
    (*pcVar1)();
    if (local_796 == 0) {
      FUN_182d_0000(64000,&local_186,unaff_SS,0x16,&local_794,unaff_SS);
    }
    FUN_184a_029c(0x16);
    if (local_796 == 0x3f) break;
    local_796 = local_796 + 1;
  }
  return;
}



// ================================================
// Function: FUN_1000_0495 at 1000:0495
// ================================================

void FUN_1000_0495(undefined4 param_1,undefined4 param_2,char param_3)

{
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  byte local_c;
  byte local_b;
  undefined1 local_a [4];
  undefined2 local_6;
  undefined2 uStack_4;
  
  uStack_4 = 0x1000;
  local_6 = 0x4a0;
  FUN_1920_04df();
  FUN_1920_090e(3,&local_6,unaff_SS,(int)param_2,(int)((ulong)param_2 >> 0x10));
  FUN_1920_090e(3,local_a,unaff_SS,(int)param_1,(int)((ulong)param_1 >> 0x10));
  local_b = *(char *)0x79cb + 1;
  if ((*(char *)0x79cb != '\0') && (*(byte *)0x79cb != 0)) {
    local_c = 1;
    while( true ) {
      if (*(char *)(local_c + 0x79cd) == param_3) {
        local_b = local_c;
      }
      if (local_c == *(byte *)0x79cb) break;
      local_c = local_c + 1;
    }
  }
  if (local_b < 3) {
    if (*(byte *)0x79cb < local_b) {
      *(char *)0x79cb = *(char *)0x79cb + '\x01';
    }
    *(char *)(local_b + 0x79cd) = param_3;
    FUN_1920_090e(3,(uint)local_b * 3 + 0x79cf,unaff_DS,&local_6,unaff_SS);
    FUN_1920_090e(3,(uint)local_b * 3 + 0x79d9,unaff_DS,local_a,unaff_SS);
  }
  return;
}



// ================================================
// Function: FUN_1000_056b at 1000:056b
// ================================================

void FUN_1000_056b(byte param_1,undefined4 param_2)

{
  bool bVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  undefined2 uVar5;
  undefined2 unaff_DS;
  undefined2 local_e;
  undefined2 local_a;
  undefined2 local_4;
  
  FUN_1920_04df();
  local_4 = 1;
  bVar1 = false;
  uVar5 = (undefined2)((ulong)param_2 >> 0x10);
  iVar2 = (int)param_2;
  *(undefined1 *)(iVar2 + 2) = 0;
  while ((local_4 <= (int)(uint)*(byte *)0x79a5 && (!bVar1))) {
    iVar3 = local_4 * 7;
    if ((*(byte *)(iVar3 + 0x771d) & param_1) != 0) {
      iVar4 = (uint)*(byte *)(iVar2 + 1) * 8;
      _local_e = (undefined2 *)CONCAT22(unaff_DS,(undefined2 *)(iVar4 + -0x3de2));
      *_local_e = *(undefined2 *)(iVar3 + 0x7719);
      *(undefined2 *)(iVar4 + -0x3de0) = *(undefined2 *)(iVar3 + 0x771b);
      bVar1 = true;
    }
    local_4 = local_4 + 1;
  }
  if (!bVar1) {
    iVar2 = (uint)*(byte *)(iVar2 + 1) * 8;
    _local_a = (undefined2 *)CONCAT22(unaff_DS,(undefined2 *)(iVar2 + -0x3de2));
    *_local_a = 0x11;
    *(undefined2 *)(iVar2 + -0x3de0) = 0x11;
  }
  return;
}



// ================================================
// Function: FUN_1000_0630 at 1000:0630
// ================================================

void __cdecl16near FUN_1000_0630(void)

{
  int iVar1;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  int local_84;
  undefined1 local_82 [124];
  undefined2 uStack_6;
  undefined2 uStack_4;
  
  uStack_4 = 0x1000;
  uStack_6 = 0x63b;
  FUN_1920_04df();
  FUN_1920_15ca(0x625,0x1920,local_82,unaff_SS);
  FUN_1920_15f8(1,local_82,unaff_SS);
  FUN_1920_16e3(0,0,2,&local_84,unaff_SS,local_82,unaff_SS);
  FUN_1920_16e3(0,0,local_84 * 6,(int)*(undefined4 *)0x79c0,
                (int)((ulong)*(undefined4 *)0x79c0 >> 0x10),local_82,unaff_SS);
  FUN_1920_1679(local_82,unaff_SS);
  iVar1 = FUN_1920_04a2();
  if (iVar1 != 0) {
    FUN_1000_00a3(1);
  }
  return;
}



// ================================================
// Function: FUN_1000_06ab at 1000:06ab
// ================================================

void FUN_1000_06ab(undefined1 param_1,undefined1 param_2,undefined1 param_3,undefined1 param_4,
                  undefined1 *param_5)

{
  undefined1 *puVar1;
  undefined2 uVar2;
  
  FUN_1920_04df();
  uVar2 = (undefined2)((ulong)param_5 >> 0x10);
  puVar1 = (undefined1 *)param_5;
  *param_5 = param_4;
  puVar1[1] = param_4;
  puVar1[2] = param_3;
  puVar1[4] = param_2;
  puVar1[5] = param_1;
  puVar1[6] = 1;
  puVar1[3] = param_2;
  return;
}



// ================================================
// Function: FUN_1000_0709 at 1000:0709
// ================================================

void FUN_1000_0709(undefined4 param_1)

{
  code *pcVar1;
  int iVar2;
  undefined2 unaff_SS;
  undefined1 local_484 [770];
  undefined1 local_182 [128];
  undefined1 local_102 [252];
  undefined2 uStack_6;
  undefined2 uStack_4;
  
  uStack_4 = 0x1000;
  uStack_6 = 0x714;
  FUN_1920_04df();
  FUN_1920_09f4(0xff,local_102,unaff_SS,(int)param_1,(int)((ulong)param_1 >> 0x10));
  FUN_1920_15ca(local_102,unaff_SS,local_182,unaff_SS);
  FUN_1920_15f8(1,local_182,unaff_SS);
  FUN_1920_16e3(0,0,0x300,local_484,unaff_SS,local_182,unaff_SS);
  FUN_1920_1679(local_182,unaff_SS);
  iVar2 = FUN_1920_04a2();
  if (iVar2 != 0) {
    FUN_1000_00a3(1);
  }
  pcVar1 = (code *)swi(0x10);
  (*pcVar1)();
  return;
}



// ================================================
// Function: FUN_1000_079d at 1000:079d
// ================================================

void __cdecl16near FUN_1000_079d(void)

{
  char cVar1;
  int iVar2;
  char cVar3;
  undefined2 unaff_DS;
  
  iVar2 = 6;
  cVar3 = -0x1a;
  cVar1 = *(char *)0x79ad;
  do {
    out(0x3c8,cVar3);
    out(0x3c9,cVar1);
    out(0x3c9,0);
    out(0x3c9,0);
    cVar1 = cVar1 + '\a';
    if ('?' < cVar1) {
      cVar1 = '\x14';
    }
    cVar3 = cVar3 + '\x01';
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  return;
}



// ================================================
// Function: FUN_1000_07c5 at 1000:07c5
// ================================================

void FUN_1000_07c5(undefined1 param_1,int param_2,int param_3,int param_4,int param_5)

{
  undefined1 *puVar1;
  undefined1 *puVar2;
  int iVar3;
  undefined1 *puVar4;
  
  FUN_1920_04df();
  puVar2 = (undefined1 *)(param_4 * 0x140 + param_5);
  iVar3 = param_3;
  puVar4 = puVar2;
  do {
    for (; iVar3 != 0; iVar3 = iVar3 + -1) {
      puVar1 = puVar2;
      puVar2 = puVar2 + 1;
      *puVar1 = param_1;
    }
    puVar2 = puVar4 + 0x140;
    param_2 = param_2 + -1;
    iVar3 = param_3;
    puVar4 = puVar2;
  } while (param_2 != 0);
  return;
}



// ================================================
// Function: FUN_1000_07fa at 1000:07fa
// ================================================

void FUN_1000_07fa(undefined1 param_1,undefined1 param_2,int param_3,int param_4,int param_5,
                  int param_6)

{
  undefined1 extraout_AH;
  
  FUN_1920_04df();
  FUN_1000_07c5(CONCAT11(extraout_AH,param_2),param_3,param_4,param_5,param_6);
  FUN_1000_07c5(CONCAT11((char)((uint)(param_3 + -2) >> 8),param_1),param_3 + -2,param_4 + -2,
                param_5 + 1,param_6 + 1);
  return;
}



// ================================================
// Function: FUN_1000_0838 at 1000:0838
// ================================================

void FUN_1000_0838(undefined2 param_1)

{
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  *(undefined2 *)0xc1ea = param_1;
  *(int *)0xc206 = *(int *)0xc1ea * -8 + 0x140;
  *(int *)0xc1ec = *(int *)0xc1ea << 3;
  *(int *)0x2094 = (*(int *)0xc204 - *(int *)0xc1ea) * 8;
  *(int *)0x78bc = (*(int *)0xc1ea - 2U >> 1) << 3;
  *(uint *)0x78be = *(uint *)0x78bc >> 3;
  *(uint *)0xc1f4 = (*(int *)0xc1ea + -1) * -8 + 0x140U >> 1;
  *(int *)0xc1f6 = *(int *)0xc1f8 - *(int *)0xc1ec;
  return;
}



// ================================================
// Function: FUN_1000_08a5 at 1000:08a5
// ================================================

void FUN_1000_08a5(int param_1,int param_2,undefined4 param_3)

{
  char *pcVar1;
  byte bVar2;
  uint uVar3;
  int iVar4;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  int *local_192;
  byte local_18b;
  byte local_18a;
  byte local_189;
  int local_188;
  int local_186;
  uint local_184;
  undefined1 local_182 [128];
  undefined1 local_102 [252];
  undefined2 uStack_6;
  undefined2 uStack_4;
  
  uStack_4 = 0x1000;
  uStack_6 = 0x8b0;
  FUN_1920_04df();
  FUN_1920_09f4(0xff,local_102,unaff_SS,(int)param_3,(int)((ulong)param_3 >> 0x10));
  FUN_1920_15ca(local_102,unaff_SS,local_182,unaff_SS);
  FUN_1920_15f8(1,local_182,unaff_SS);
  FUN_1920_16e3(0,0,1,&local_189,unaff_SS,local_182,unaff_SS);
  FUN_1920_16e3(0,0,(uint)local_189 * 0x26,(*(byte *)0x208d + 1) * 0x26 + 0x1bae,unaff_DS,local_182,
                unaff_SS);
  local_18a = *(byte *)0xc496;
  uVar3 = (uint)local_189;
  if (uVar3 != 0) {
    local_184 = 1;
    while( true ) {
      FUN_1920_16e3(0,0,1,&local_18b,unaff_SS,local_182,unaff_SS);
      FUN_18ac_0517(1,1,local_18b);
      if (local_184 == uVar3) break;
      local_184 = local_184 + 1;
    }
  }
  uVar3 = (uint)local_189;
  if (uVar3 != 0) {
    local_184 = 1;
    while( true ) {
      FUN_1920_16e3(0,0,2,&local_186,unaff_SS,local_182,unaff_SS);
      FUN_1920_16e3(0,0,2,&local_188,unaff_SS,local_182,unaff_SS);
      iVar4 = (local_18a + local_184 + -1) * 8;
      _local_192 = (int *)CONCAT22(unaff_DS,(int *)(iVar4 + -0x3de2));
      *_local_192 = param_2 + local_186;
      *(int *)(iVar4 + -0x3de0) = param_1 + local_188;
      if (local_184 == uVar3) break;
      local_184 = local_184 + 1;
    }
  }
  FUN_1920_16e3(0,0,1,&local_18b,unaff_SS,local_182,unaff_SS);
  *(int *)((*(byte *)0x208d + 1) * 0x26 + 0x1bc0) = *(byte *)0x208d + 1;
  if (1 < local_189) {
    local_184 = 2;
    while( true ) {
      iVar4 = (*(byte *)0x208d + local_184) * 0x26;
      if ((((*(uint *)(iVar4 + 0x1bbc) & 0xff) != 0) &&
          (*(int *)(iVar4 + 0x1bbc) = *(int *)(iVar4 + 0x1bbc) + (uint)*(byte *)0x79f9,
          *(uint *)(iVar4 + 0x1bbc) >> 8 != 0)) &&
         (*(int *)(iVar4 + 0x1bbc) = *(int *)(iVar4 + 0x1bbc) + (uint)*(byte *)0x79f9 * 0x100,
         (*(uint *)(iVar4 + 0x1bbe) & 0xff) != 0)) {
        *(int *)(iVar4 + 0x1bbe) = *(int *)(iVar4 + 0x1bbe) + (uint)*(byte *)0x79f9;
      }
      *(char *)(iVar4 + 0x1bd3) = *(char *)0x208d + '\x01';
      if (local_184 == local_189) break;
      local_184 = local_184 + 1;
    }
  }
  FUN_1920_16e3(0,0,(uint)local_18b << 4,(*(byte *)0x79f9 + 1) * 0x10 + 0x79ea,unaff_DS,local_182,
                unaff_SS);
  iVar4 = FUN_1920_04a2();
  if (iVar4 != 0) {
    FUN_1000_00a3(1);
  }
  bVar2 = *(byte *)0x79f9;
  local_184 = *(byte *)0x79f9 + 1;
  if (local_184 <= (uint)bVar2 + (uint)local_18b) {
    while( true ) {
      _local_192 = (int *)CONCAT22(unaff_DS,(char *)(local_184 * 0x10 + 0x79ea));
      *(byte *)_local_192 = *(char *)_local_192 + local_18a;
      pcVar1 = (char *)(local_184 * 0x10 + 0x79eb);
      *pcVar1 = *pcVar1 + local_18a;
      if (local_184 == (uint)bVar2 + (uint)local_18b) break;
      local_184 = local_184 + 1;
    }
  }
  *(char *)0x79f9 = *(char *)0x79f9 + local_18b;
  uVar3 = (uint)local_189;
  if (uVar3 != 0) {
    local_184 = 1;
    while( true ) {
      iVar4 = (*(byte *)0x208d + local_184) * 0x26;
      if (1 < (int)local_184) {
        *(char *)(iVar4 + 0x1bb0) = *(char *)(iVar4 + 0x1bb0) + local_18a;
      }
      *(char *)(iVar4 + 0x1baf) = *(char *)(iVar4 + 0x1baf) + local_18a;
      if (*(char *)(iVar4 + 0x1bb1) != '\0') {
        FUN_1000_06ab(1,*(undefined1 *)(iVar4 + 0x1bc8),
                      *(undefined1 *)((uint)*(byte *)(iVar4 + 0x1bb1) * 2 + 0x59),
                      *(undefined1 *)((uint)*(byte *)(iVar4 + 0x1bb1) * 2 + 0x58),iVar4 + 0x1bc4,
                      unaff_DS);
      }
      if (local_184 == uVar3) break;
      local_184 = local_184 + 1;
    }
  }
  *(char *)0x208d = *(char *)0x208d + local_189;
  return;
}



// ================================================
// Function: FUN_1000_0c33 at 1000:0c33
// ================================================

void __cdecl16near FUN_1000_0c33(void)

{
  undefined2 uVar1;
  int iVar2;
  int iVar3;
  int extraout_DX;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined4 uVar4;
  undefined2 local_e;
  undefined2 local_c;
  undefined2 local_a;
  undefined2 local_8;
  int local_6;
  undefined2 local_4;
  
  local_4 = 0x1000;
  local_6 = 0xc3e;
  FUN_1920_04df();
  FUN_1920_177b(0x7eda,unaff_DS);
  iVar3 = extraout_DX;
  iVar2 = FUN_1920_04a9();
  if ((*(int *)((uint)*(byte *)0x79b7 * 4 + 0x1a88) != iVar3) ||
     (*(int *)((uint)*(byte *)0x79b7 * 4 + 0x1a86) != iVar2)) {
    FUN_1920_174b(*(undefined2 *)((uint)*(byte *)0x79b7 * 4 + 0x1a86),
                  *(undefined2 *)((uint)*(byte *)0x79b7 * 4 + 0x1a88),0x7eda,unaff_DS);
    FUN_1920_04a9();
  }
  if (*(int *)0x6616 != 0 || *(int *)0x6618 != 0) {
    FUN_1920_0254(*(int *)0x78ba * 2 + 0x10,*(undefined2 *)0x6616,*(undefined2 *)0x6618);
    *(undefined2 *)0x6616 = 0;
    *(undefined2 *)0x6618 = 0;
  }
  if (*(int *)0x661a != 0 || *(int *)0x661c != 0) {
    FUN_1920_0254(*(int *)0x78ba + 0x10,*(undefined2 *)0x661a,*(undefined2 *)0x661c);
    *(undefined2 *)0x661a = 0;
    *(undefined2 *)0x661c = 0;
  }
  FUN_1920_16e3(0,0,2,&local_6,unaff_SS,0x7eda,unaff_DS);
  *(int *)0xc204 = local_6;
  FUN_1920_16e3(0,0,2,&local_6,unaff_SS,0x7eda,unaff_DS);
  *(int *)0x2096 = (local_6 + -0x15) * 8;
  *(int *)0x78ba = local_6 * *(int *)0xc204;
  FUN_1920_16e3(0,0,1,0x79b4,unaff_DS,0x7eda,unaff_DS);
  FUN_1920_16e3(0,0,2,0x2086,unaff_DS,0x7eda,unaff_DS);
  FUN_1920_16e3(0,0,1,0x79b3,unaff_DS,0x7eda,unaff_DS);
  uVar4 = FUN_1920_023f(*(int *)0x78ba + 0x10);
  *(undefined2 *)0xc1e0 = (int)uVar4;
  *(undefined2 *)0xc1e2 = (int)((ulong)uVar4 >> 0x10);
  uVar1 = *(undefined2 *)0xc1e2;
  *(undefined2 *)0x661a = *(undefined2 *)0xc1e0;
  *(undefined2 *)0x661c = uVar1;
  if ((int)*(undefined4 *)0xc1e0 != 0) {
    if ((uint)*(undefined4 *)0xc1e0 < 0x10) {
      uVar4 = *(undefined4 *)0xc1e0;
      *(undefined2 *)0xc1e0 = 0;
      *(int *)0xc1e2 = (int)((ulong)uVar4 >> 0x10) + 1;
    }
    else {
      FUN_1920_00e9();
    }
  }
  uVar4 = FUN_1920_023f(*(int *)0x78ba * 2 + 0x10);
  *(undefined2 *)0x6612 = (int)uVar4;
  *(undefined2 *)0x6614 = (int)((ulong)uVar4 >> 0x10);
  uVar1 = *(undefined2 *)0x6614;
  *(undefined2 *)0x6616 = *(undefined2 *)0x6612;
  *(undefined2 *)0x6618 = uVar1;
  if ((int)*(undefined4 *)0x6612 != 0) {
    if ((uint)*(undefined4 *)0x6612 < 0x10) {
      uVar4 = *(undefined4 *)0x6612;
      *(undefined2 *)0x6612 = 0;
      *(int *)0x6614 = (int)((ulong)uVar4 >> 0x10) + 1;
    }
    else {
      FUN_1920_00e9();
    }
  }
  *(undefined2 *)0xc1fe = (int)((ulong)*(undefined4 *)0xc1e0 >> 0x10);
  FUN_1920_16e3(0,0,2,&local_4,unaff_SS,0x7eda,unaff_DS);
  FUN_1920_16e3(0,0,local_4,(int)*(undefined4 *)0xc498,(int)((ulong)*(undefined4 *)0xc498 >> 0x10),
                0x7eda,unaff_DS);
  local_a = *(undefined2 *)0xc498;
  local_8 = *(undefined2 *)0xc49a;
  local_e = *(undefined2 *)0xc1e0;
  local_c = *(undefined2 *)0xc1e2;
  FUN_182d_0000(*(undefined2 *)0x78ba,&local_e,unaff_SS,local_4,&local_a,unaff_SS);
  FUN_1920_16e3(0,0,2,&local_4,unaff_SS,0x7eda,unaff_DS);
  FUN_1920_16e3(0,0,local_4,(int)*(undefined4 *)0xc498,(int)((ulong)*(undefined4 *)0xc498 >> 0x10),
                0x7eda,unaff_DS);
  *(int *)0x78ba = *(int *)0x78ba << 1;
  local_e = *(undefined2 *)0x6612;
  local_c = *(undefined2 *)0x6614;
  FUN_182d_0000(*(undefined2 *)0x78ba,&local_e,unaff_SS,local_4,&local_a,unaff_SS);
  *(uint *)0x78ba = *(uint *)0x78ba >> 1;
  FUN_1920_16e3(0,0,2,0x78c4,unaff_DS,0x7eda,unaff_DS);
  FUN_1920_16e3(0,0,2,0x78c6,unaff_DS,0x7eda,unaff_DS);
  FUN_1920_16e3(0,0,1,0x79a6,unaff_DS,0x7eda,unaff_DS);
  if (*(char *)0x79a6 != '\0') {
    FUN_1920_16e3(0,0,(uint)*(byte *)0x79a6 * 0x1e,0x74c6,unaff_DS,0x7eda,unaff_DS);
  }
  FUN_1920_16e3(0,0,1,0x79a5,unaff_DS,0x7eda,unaff_DS);
  if (*(char *)0x79a5 != '\0') {
    FUN_1920_16e3(0,0,(uint)*(byte *)0x79a5 * 7,0x771e,unaff_DS,0x7eda,unaff_DS);
  }
  FUN_1920_16e3(0,0,1,0x79a7,unaff_DS,0x7eda,unaff_DS);
  if (*(char *)0x79a7 != '\0') {
    FUN_1920_16e3(0,0,(uint)*(byte *)0x79a7 * 0xe,0x77ce,unaff_DS,0x7eda,unaff_DS);
  }
  iVar3 = FUN_1920_04a2();
  if (iVar3 != 0) {
    FUN_1000_00a3(1);
  }
  return;
}



// ================================================
// Function: FUN_1000_0faa at 1000:0faa
// ================================================

void __cdecl16near FUN_1000_0faa(void)

{
  FUN_1920_04df();
  FUN_1000_0c33();
  return;
}



// ================================================
// Function: FUN_1000_1156 at 1000:1156
// ================================================

void __cdecl16near FUN_1000_1156(void)

{
  undefined2 *puVar1;
  undefined2 *puVar2;
  int iVar3;
  int iVar4;
  undefined2 *puVar5;
  undefined2 *puVar6;
  undefined2 unaff_DS;
  
  puVar6 = (undefined2 *)*(undefined2 *)0x2072;
  iVar3 = 8;
  puVar5 = (undefined2 *)*(undefined2 *)0x2074;
  do {
    for (iVar4 = 4; iVar4 != 0; iVar4 = iVar4 + -1) {
      puVar2 = puVar6;
      puVar6 = puVar6 + 1;
      puVar1 = puVar5;
      puVar5 = puVar5 + 1;
      *puVar2 = *puVar1;
    }
    puVar6 = puVar6 + 0x9c;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  return;
}



// ================================================
// Function: FUN_1000_1174 at 1000:1174
// ================================================

void FUN_1000_1174(int param_1)

{
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  *(int *)0x2072 = param_1 + 0x501;
  *(int *)0x2074 = (uint)*(byte *)0x79b4 * 0x40 + *(int *)0xc208;
  FUN_1000_1156();
  *(int *)0x2072 = *(int *)0x2072 + 0x1400;
  *(int *)0x2074 = *(int *)0xc208 + 0x1d40;
  FUN_1000_1156();
  return;
}



// ================================================
// Function: FUN_1000_11b0 at 1000:11b0
// ================================================

void FUN_1000_11b0(undefined4 param_1)

{
  int iVar1;
  byte bVar2;
  int iVar3;
  undefined2 uVar4;
  undefined2 unaff_DS;
  undefined2 local_8;
  undefined2 local_4;
  
  FUN_1920_04df();
  uVar4 = (undefined2)((ulong)param_1 >> 0x10);
  iVar3 = (int)param_1;
  local_4 = *(int *)(iVar3 + 2);
  local_8 = 0;
  do {
    bVar2 = 10;
    FUN_1920_0945();
    *(int *)(iVar3 + local_8 * 2 + 0x18) = (uint)bVar2 * 0x40 + *(int *)0x2070;
    iVar1 = FUN_1920_0945();
    local_8 = local_8 + 1;
    if (iVar1 == 0 && local_4 == 0) break;
  } while (local_8 != 8);
  *(undefined1 *)(iVar3 + 0x2c) = 1;
  return;
}



// ================================================
// Function: FUN_1000_1239 at 1000:1239
// ================================================

void FUN_1000_1239(int param_1)

{
  bool bVar1;
  int iVar2;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  *(undefined2 *)0x2072 = *(undefined2 *)0x2074;
  param_1 = param_1 + 0x10;
  iVar2 = 9;
  bVar1 = false;
  do {
    if ((bVar1) || (*(int *)(param_1 + 0x785e) != *(int *)0x2070)) {
      *(int *)0x2074 = *(int *)(param_1 + 0x785e);
      FUN_1000_1156();
      bVar1 = true;
    }
    *(int *)0x2072 = *(int *)0x2072 + 9;
    param_1 = param_1 + -2;
    if (iVar2 == 2) {
      bVar1 = true;
    }
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  return;
}



// ================================================
// Function: FUN_1000_1287 at 1000:1287
// ================================================

void FUN_1000_1287(int param_1)

{
  int iVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  char cVar5;
  int iVar6;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  iVar6 = param_1 + 0x10;
  cVar5 = '\0';
  uVar4 = 9;
  do {
    iVar1 = *(int *)(iVar6 + 0x785e);
    if (iVar1 != *(int *)(iVar6 + 0x7872)) {
      uVar2 = iVar1 + 8;
      if ((uVar4 & 1) != 0) {
        uVar2 = iVar1 + 0x10;
      }
      cVar5 = cVar5 + '\x01';
      uVar3 = uVar2 - 0x280;
      if (uVar2 - 0x280 < *(uint *)0x2070) {
        uVar3 = uVar2;
      }
      *(uint *)(iVar6 + 0x785e) = uVar3;
    }
    iVar6 = iVar6 + -2;
    uVar4 = uVar4 - 1;
  } while (uVar4 != 0);
  if (cVar5 == '\0') {
    *(undefined1 *)(param_1 + 0x7886) = 2;
  }
  return;
}



// ================================================
// Function: FUN_1000_12db at 1000:12db
// ================================================

void FUN_1000_12db(byte param_1,int param_2,int param_3)

{
  char *pcVar1;
  char cVar2;
  byte bVar3;
  undefined1 uVar4;
  undefined2 uVar5;
  char cVar6;
  uint uVar7;
  uint uVar8;
  char *pcVar9;
  int iVar10;
  undefined1 *puVar11;
  undefined1 *puVar12;
  undefined2 unaff_DS;
  byte *local_a;
  
  FUN_1920_04df();
  iVar10 = (uint)param_1 * 4;
  _local_a = (byte *)CONCAT22(unaff_DS,(byte *)(iVar10 + -0x3cde));
  pcVar9 = (char *)*(int *)(iVar10 + -0x3cdc);
  bVar3 = *_local_a;
  puVar11 = (undefined1 *)(param_3 + param_2 * 0x140);
  uVar8 = (uint)*(byte *)(iVar10 + -0x3cdd);
  iVar10 = (uint)*(byte *)0x79a3 << 8;
  uVar4 = *(undefined1 *)0x661e;
  uVar5 = *(undefined2 *)0xc1fa;
  do {
    uVar7 = (uint)bVar3;
    puVar12 = puVar11;
    do {
      pcVar1 = pcVar9;
      pcVar9 = pcVar9 + 1;
      cVar2 = *pcVar1;
      cVar6 = (char)((uint)iVar10 >> 8);
      iVar10 = CONCAT11(cVar6,cVar2);
      if (cVar2 == '\0') {
        iVar10 = CONCAT11(cVar6,cVar6);
        if (cVar6 != '\0') goto LAB_1000_135e;
      }
      else {
        if (cVar2 == '\x01') {
          iVar10 = CONCAT11(cVar6,uVar4);
        }
LAB_1000_135e:
        *puVar12 = (char)iVar10;
      }
      puVar12 = puVar12 + 1;
      uVar7 = uVar7 - 1;
    } while (uVar7 != 0);
    puVar11 = puVar11 + 0x140;
    uVar8 = uVar8 - 1;
    if (uVar8 == 0) {
      return;
    }
  } while( true );
}



// ================================================
// Function: FUN_1000_136e at 1000:136e
// ================================================

void FUN_1000_136e(byte param_1,byte param_2,char param_3,undefined1 param_4,undefined4 param_5,
                  int param_6,int param_7,int param_8,int param_9)

{
  byte bVar1;
  int iVar2;
  int iVar3;
  undefined1 uVar4;
  undefined1 extraout_AH;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  int local_108;
  uint local_104;
  byte local_102 [252];
  undefined2 uStack_6;
  undefined2 uStack_4;
  
  uStack_4 = 0x1000;
  uStack_6 = 0x1379;
  FUN_1920_04df();
  FUN_1920_09f4(0xff,local_102,unaff_SS,(int)param_5,(int)((ulong)param_5 >> 0x10));
  *(undefined1 *)0x79a3 = 0;
  if (local_102[0] != 0) {
    local_104 = 1;
    while( true ) {
      bVar1 = local_102[local_104];
      iVar2 = (local_104 - 1) * (uint)param_2 + param_9;
      if (bVar1 != 0x20) {
        iVar3 = (bVar1 - 0x61) + (uint)param_1;
        local_108 = iVar3;
        if ((0x2f < bVar1) && (bVar1 < 0x40)) {
          iVar3 = param_1 + 0x1a;
          local_108 = (bVar1 - 0x30) + iVar3;
        }
        uVar4 = (undefined1)((uint)iVar3 >> 8);
        if (param_3 != '\0') {
          *(char *)0x661e = param_3;
          FUN_1000_12db(CONCAT11((char)((uint)(param_8 + param_6) >> 8),(undefined1)local_108),
                        param_8 + param_6,iVar2 + param_7);
          uVar4 = extraout_AH;
        }
        *(undefined1 *)0x661e = param_4;
        FUN_1000_12db(CONCAT11(uVar4,(undefined1)local_108),param_8,iVar2);
      }
      if (local_104 == local_102[0]) break;
      local_104 = local_104 + 1;
    }
  }
  return;
}



// ================================================
// Function: FUN_1000_146a at 1000:146a
// ================================================

void FUN_1000_146a(char param_1,undefined2 param_2,undefined1 param_3,byte param_4,
                  undefined1 param_5,byte param_6,byte param_7,undefined4 param_8,undefined2 param_9
                  ,int param_10)

{
  char cVar1;
  undefined1 uVar2;
  int iVar3;
  int iVar4;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined1 *puVar5;
  undefined2 uVar6;
  undefined1 *puVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined1 local_314 [252];
  undefined1 local_218 [2];
  undefined1 local_216 [2];
  undefined1 local_214 [252];
  int local_118;
  int local_116;
  undefined1 local_113;
  int local_112;
  int local_110;
  int local_10e;
  int local_10c;
  undefined1 local_10a [8];
  byte local_102 [252];
  undefined2 uStack_6;
  undefined2 uStack_4;
  
  uStack_4 = 0x1000;
  uStack_6 = 0x1475;
  FUN_1920_04df();
  FUN_1920_09f4(0xff,local_102,unaff_SS,(int)param_8,(int)((ulong)param_8 >> 0x10));
  *(undefined1 *)0x79ba = 0;
  if (param_1 != '\0') {
    param_10 = 0xa0 - ((uint)local_102[0] * (uint)param_4 >> 1);
  }
  FUN_1920_09f4(6,local_10a,unaff_SS,0x1463,0x1920);
  local_112 = ((uint)param_6 - (uint)param_7) + 1;
  if (local_112 < 7) {
    local_10c = param_10 - (((uint)param_6 - (uint)param_7) + 2) * (uint)param_4;
    puVar5 = local_214;
    uVar10 = unaff_SS;
    FUN_1920_0a26(local_112,1,local_10a,unaff_SS);
    FUN_1920_0a67(local_102,unaff_SS);
    puVar7 = local_314;
    uVar6 = unaff_SS;
    FUN_1920_0a26(local_112,1,local_10a,unaff_SS);
    FUN_1920_0a67(puVar7,uVar6);
    FUN_1920_09f4(0xff,local_102,unaff_SS,puVar5,uVar10);
    local_116 = (uint)local_102[0] - local_112;
    if (0 < local_116) {
      local_10e = 1;
      while( true ) {
        puVar5 = local_216;
        uVar6 = unaff_SS;
        FUN_1920_0a26(local_112,local_10e,local_102,unaff_SS);
        FUN_1920_09f4(6,local_10a,unaff_SS,puVar5,uVar6);
        local_118 = local_112;
        if (0 < local_112) {
          local_110 = 1;
          while( true ) {
            if (local_110 == local_112) {
              local_113 = param_5;
            }
            else {
              local_113 = 0;
            }
            iVar3 = (uint)param_4 * local_110 + local_10c;
            uVar9 = 0xffff;
            uVar8 = 0xffff;
            puVar5 = local_218;
            uVar6 = unaff_SS;
            uVar10 = param_9;
            FUN_1920_0b0a(CONCAT11((char)((uint)iVar3 >> 8),local_10a[local_110]));
            iVar4 = ((uint)param_6 - local_110) + 1;
            uVar2 = (undefined1)((uint)iVar4 >> 8);
            FUN_1000_136e(CONCAT11(uVar2,param_3),CONCAT11(uVar2,param_4),CONCAT11(uVar2,local_113),
                          iVar4,puVar5,uVar6,uVar8,uVar9,uVar10,iVar3);
            if (local_110 == local_118) break;
            local_110 = local_110 + 1;
          }
        }
        FUN_184a_029c(param_2);
        cVar1 = FUN_184a_02fd();
        if (cVar1 != '\0') {
          param_2 = 0;
          uVar2 = FUN_184a_030f();
          *(undefined1 *)0x2058 = uVar2;
          *(undefined1 *)0x79ba = 2;
          if (*(char *)0x2058 == '\x1b') {
            *(undefined1 *)0x79ba = 1;
          }
        }
        local_10c = local_10c + (uint)param_4;
        if (local_10e == local_116) break;
        local_10e = local_10e + 1;
      }
    }
  }
  return;
}



// ================================================
// Function: FUN_1000_165a at 1000:165a
// ================================================

void __cdecl16near FUN_1000_165a(void)

{
  undefined2 unaff_DS;
  
  if ((*(char *)0x79c4 == '\0') || ((char)(*(char *)0x799e + -1) < *(char *)0x799f)) {
    *(undefined1 *)0x799e = *(undefined1 *)0x799f;
    *(undefined2 *)0x78c0 = *(undefined2 *)0x2074;
    *(undefined1 *)0x79c4 = 1;
  }
  return;
}



// ================================================
// Function: FUN_1000_16ca at 1000:16ca
// ================================================

void __cdecl16near FUN_1000_16ca(void)

{
  undefined1 uVar1;
  uint uVar2;
  int iVar3;
  undefined2 uVar4;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined1 local_84;
  byte local_83;
  undefined1 local_82 [124];
  undefined2 uStack_6;
  undefined2 uStack_4;
  
  uStack_4 = 0x1000;
  uVar4 = 0x1920;
  uStack_6 = 0x16d5;
  FUN_1920_04df();
  do {
    FUN_1920_15ca(0x167e,uVar4,local_82,unaff_SS);
    FUN_1920_1601(1,local_82,unaff_SS);
    local_84 = 7;
    FUN_1920_16ea(0,0,1,&local_84,unaff_SS,local_82,unaff_SS);
    local_83 = 1;
    while( true ) {
      uVar2 = (uint)local_83;
      FUN_1920_16ea(0,0,4,uVar2 * 0xd + 0x1af7,unaff_DS,local_82,unaff_SS);
      FUN_1920_16ea(0,0,9,uVar2 * 0xd + 0x1afb,unaff_DS,local_82,unaff_SS);
      if (local_83 == 7) break;
      local_83 = local_83 + 1;
    }
    FUN_1920_1679(local_82,unaff_SS);
    iVar3 = FUN_1920_04a2();
    if (iVar3 == 0) {
      return;
    }
    FUN_1000_07c5(0,200,0x140,0,0);
    FUN_1000_136e(0x1b,9,0xd,0x37,0x1687,0x1920,1,1,0x59,10);
    FUN_1000_136e(0x1b,9,0xd,0x37,0x169a,0x1920,1,1,100,10);
    FUN_1000_136e(0x1b,9,0xd,0x37,0x16ae,0x1920,1,1,0x6f,10);
    do {
      uVar4 = 0x184a;
      uVar1 = FUN_184a_030f();
      *(undefined1 *)0x2058 = uVar1;
      if (*(char *)0x2058 == '\x1b') break;
    } while (*(char *)0x2058 != '\r');
    if (*(char *)0x2058 != '\r') {
      return;
    }
  } while( true );
}



// ================================================
// Function: FUN_1000_1845 at 1000:1845
// ================================================

void FUN_1000_1845(undefined2 param_1,int param_2,uint param_3,uint param_4)

{
  char cVar1;
  byte bVar2;
  undefined1 uVar3;
  undefined1 extraout_AH;
  uint uVar4;
  int iVar5;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined1 *puVar6;
  undefined2 uVar7;
  undefined1 *puVar8;
  undefined2 uVar9;
  undefined2 uVar10;
  undefined2 uVar11;
  undefined1 local_212 [256];
  undefined1 local_112 [252];
  uint *local_16;
  undefined1 local_12 [10];
  undefined1 local_8 [2];
  undefined2 uStack_6;
  undefined2 local_4;
  
  local_4 = 0x1000;
  uStack_6 = 0x1850;
  FUN_1920_04df();
  FUN_1000_01fc();
  *(undefined2 *)0x2074 = 0x78;
  *(undefined1 *)0x799f = 0xb;
  FUN_1000_165a();
  for (local_4._1_1_ = 7; local_4._1_1_ != 0; local_4._1_1_ = local_4._1_1_ - 1) {
    iVar5 = *(int *)((uint)local_4._1_1_ * 0xd + 0x1af9);
    if (((int)param_4 <= iVar5) &&
       (((int)param_4 < iVar5 || (param_3 <= *(uint *)((uint)local_4._1_1_ * 0xd + 0x1af7)))))
    break;
  }
  local_4._1_1_ = local_4._1_1_ + 1;
  FUN_1920_154e(2,local_8,unaff_SS,0,local_4._1_1_,0);
  uVar11 = 0x3c;
  uVar9 = 10;
  puVar8 = local_212;
  uVar10 = unaff_SS;
  FUN_1920_09da(0x17f2,0x1920);
  puVar6 = local_112;
  uVar7 = unaff_SS;
  FUN_1920_0b0a(CONCAT11(extraout_AH,*(undefined1 *)(param_2 + 0x1abe)));
  FUN_1920_0a67(puVar6,uVar7);
  FUN_1000_146a(1,*(undefined2 *)0x78ce,0x1b,0xb,0x19,0x1f,0x1b,puVar8,uVar10,uVar9,uVar11);
  uVar9 = 0x3c;
  uVar10 = 0x15;
  puVar8 = local_112;
  uVar7 = unaff_SS;
  FUN_1920_09da(0x17fd,0x1920);
  FUN_1920_0a67(local_8,unaff_SS);
  FUN_1920_0a67(0x180e,0x1920);
  FUN_1000_146a(1,*(undefined2 *)0x78ce,0x1b,0xb,0x19,0x1f,0x1b,puVar8,uVar7,uVar10,uVar9);
  FUN_1000_146a(1,*(undefined2 *)0x78ce,0x1b,0xb,0x19,0x1f,0x1b,0x1815,0x1920,0x20,0x3c);
  FUN_1000_146a(1,*(undefined2 *)0x78ce,0x1b,0xb,0x19,0x1f,0x1b,0x1825,0x1920,0x2b,0x50);
  FUN_1000_07fa(8,7,0xd,0x4c,0x3a,0x7a);
  FUN_1920_09f4(8,local_12,unaff_SS,0x183c,0x1920);
  local_4 = CONCAT11(local_4._1_1_,1);
  do {
    FUN_1000_136e(0x1b,9,0x37,0x4d,local_12,unaff_SS,1,1,0x3c,0x7c);
    uVar3 = FUN_184a_030f();
    *(undefined1 *)0x2058 = uVar3;
    uVar3 = FUN_1920_1807(*(undefined1 *)0x2058);
    *(undefined1 *)0x2058 = uVar3;
    if ((((*(byte *)0x2058 < 0x41) || (0x5a < *(byte *)0x2058)) && (*(char *)0x2058 != ' ')) ||
       (8 < (byte)local_4)) {
      if ((*(char *)0x2058 == '\b') && (1 < (byte)local_4)) {
        (&stack0xffed)[local_4 & 0xff] = 0x3a;
        bVar2 = (byte)local_4 - 1;
        local_4 = CONCAT11(local_4._1_1_,bVar2);
        FUN_1000_07c5(8,10,10,0x3c,(uint)bVar2 * 9 + 0x73);
      }
    }
    else {
      *(byte *)0x2058 = *(byte *)0x2058 | 0x20;
      local_12[local_4 & 0xff] = *(undefined1 *)0x2058;
      FUN_1000_07c5(8,10,10,0x3c,(local_4 & 0xff) * 9 + 0x73);
      cVar1 = (byte)local_4 + '\x01';
      local_4 = CONCAT11(local_4._1_1_,cVar1);
      *(undefined2 *)0x2074 = 0;
      *(char *)0x799f = cVar1;
      FUN_1000_165a();
    }
  } while (*(char *)0x2058 != '\r');
  *(undefined2 *)0x2074 = 8;
  *(undefined1 *)0x799f = 0xb;
  FUN_1000_165a();
  if ((local_4._1_1_ < 7) && (uVar4 = (uint)local_4._1_1_, uVar4 + 1 < 8)) {
    *(undefined2 *)0x2082 = 7;
    while (FUN_1920_090e(0xd,*(int *)0x2082 * 0xd + 0x1af7,unaff_DS,
                         (*(int *)0x2082 + -1) * 0xd + 0x1af7,unaff_DS),
          *(uint *)0x2082 != uVar4 + 1) {
      *(int *)0x2082 = *(int *)0x2082 + -1;
    }
  }
  iVar5 = (local_4 >> 8) * 0xd;
  _local_16 = (uint *)CONCAT22(unaff_DS,(uint *)(iVar5 + 0x1af7));
  FUN_1920_09f4(8,iVar5 + 0x1afb,unaff_DS,local_12,unaff_SS);
  *_local_16 = param_3;
  ((uint *)_local_16)[1] = param_4;
  FUN_1000_16ca();
  return;
}



// ================================================
// Function: FUN_1000_1b14 at 1000:1b14
// ================================================

void FUN_1000_1b14(char param_1)

{
  uint uVar1;
  uint *puVar2;
  undefined2 *puVar3;
  undefined1 uVar4;
  int iVar5;
  undefined1 extraout_AH;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined1 *puVar6;
  undefined2 uVar7;
  undefined1 *puVar8;
  undefined2 uVar9;
  undefined1 local_30e [256];
  undefined1 local_20e [252];
  uint auStack_112 [6];
  undefined1 local_106 [256];
  int local_6;
  int local_4;
  
  local_4 = 0x1000;
  local_6 = 0x1b1f;
  FUN_1920_04df();
  FUN_1000_01fc();
  FUN_1920_1679(0x7eda,unaff_DS);
  FUN_1920_04a9();
  if (*(char *)0x79f8 != '\x01') {
    FUN_18ac_05dc(0x1ad7,0x1920);
    iVar5 = FUN_1920_04a2();
    if (iVar5 != 0) {
      FUN_1000_00a3(1);
    }
    *(undefined1 *)0x79f8 = 1;
  }
  local_4 = 0x4d;
  *(undefined1 *)0x208c = 0;
  if (param_1 == '\x01') {
    FUN_1000_146a(1,*(undefined2 *)0x78ce,1,0xb,0x19,0x1f,0x1b,0x1ae1,0x1920,0x4d,0x3c);
  }
  else if (param_1 == '\x02') {
    FUN_1000_146a(1,*(undefined2 *)0x78ce,0x1b,0xb,0x19,0x1f,0x1b,0x1aeb,0x1920,0x37,0x3c);
    FUN_1000_146a(1,*(undefined2 *)0x78ce,1,0xb,0x19,0x1f,0x1b,0x1af9,0x1920,0x42,0x3c);
    *(undefined1 *)0x208c = 1;
  }
  local_4 = local_4 + 0x14;
  FUN_1000_146a(1,*(undefined2 *)0x78ce,1,0xb,0x19,0xfff4,0xfff0,
                (uint)*(byte *)0x79cc * 0xd00 + 0x98a,unaff_DS,local_4,0x3c);
  local_4 = local_4 + 0x18;
  local_6 = 1;
  while( true ) {
    iVar5 = local_6;
    if (local_6 == 1) {
      *(undefined2 *)0x78b6 = 0x785a;
      *(undefined2 *)0x78b8 = unaff_DS;
      *(undefined1 *)0x2058 = 0x31;
    }
    else {
      *(undefined2 *)0x78b6 = 0x7888;
      *(undefined2 *)0x78b8 = unaff_DS;
      *(undefined1 *)0x2058 = 0x32;
    }
    puVar2 = (uint *)*(undefined4 *)0x78b6;
    uVar1 = ((uint *)puVar2)[1];
    auStack_112[local_6 * 2] = *puVar2;
    auStack_112[iVar5 * 2 + 1] = uVar1;
    if ((0 < (int)auStack_112[local_6 * 2 + 1]) ||
       ((-1 < (int)auStack_112[local_6 * 2 + 1] && (auStack_112[local_6 * 2] != 0)))) {
      puVar3 = (undefined2 *)*(undefined4 *)0x78b6;
      FUN_1920_154e(0xff,local_106,unaff_SS,0,*puVar3,((undefined2 *)puVar3)[1]);
      puVar8 = local_30e;
      uVar9 = unaff_SS;
      FUN_1920_09da((uint)*(byte *)0x79cc * 0xd00 + 0xa8a,unaff_DS);
      puVar6 = local_20e;
      uVar7 = unaff_SS;
      FUN_1920_0b0a(CONCAT11(extraout_AH,*(undefined1 *)0x2058));
      FUN_1920_0a67(puVar6,uVar7);
      FUN_1920_0a67(0x1b11,0x1920);
      FUN_1920_0a67(local_106,unaff_SS);
      FUN_1920_09f4(0xff,local_106,unaff_SS,puVar8,uVar9);
      FUN_1000_146a(1,*(undefined2 *)0x78ce,0x1b,9,0x19,0xfff4,0xfff0,local_106,unaff_SS,local_4,
                    0x3c);
      local_4 = local_4 + 0xb;
    }
    if (local_6 == 2) break;
    local_6 = local_6 + 1;
  }
  uVar4 = FUN_184a_030f();
  *(undefined1 *)0x2058 = uVar4;
  local_6 = 1;
  while( true ) {
    if ((*(int *)0x1b54 < (int)auStack_112[local_6 * 2 + 1]) ||
       ((*(int *)0x1b54 <= (int)auStack_112[local_6 * 2 + 1] &&
        (*(uint *)0x1b52 <= auStack_112[local_6 * 2])))) {
      FUN_1000_1845(&stack0xfffe,local_6,auStack_112[local_6 * 2],auStack_112[local_6 * 2 + 1]);
    }
    if (local_6 == 2) break;
    local_6 = local_6 + 1;
  }
  return;
}



// ================================================
// Function: FUN_1000_1d61 at 1000:1d61
// ================================================

void __cdecl16near FUN_1000_1d61(void)

{
  uint *puVar1;
  char cVar2;
  undefined1 uVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  undefined2 uVar8;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined1 *puVar9;
  undefined2 uVar10;
  undefined1 local_318 [256];
  int local_218;
  uint *local_216;
  undefined1 local_20a [256];
  undefined1 local_10a [258];
  undefined2 local_8;
  int local_6;
  int local_4;
  
  local_4 = 0x1000;
  local_6 = 0x1d6c;
  FUN_1920_04df();
  if (*(char *)0x79f8 != '\x01') {
    FUN_18ac_05dc(0x1d45,0x1920);
    iVar4 = FUN_1920_04a2();
    if (iVar4 != 0) {
      FUN_1000_00a3(1);
    }
    *(undefined1 *)0x79f8 = 1;
  }
  FUN_184a_029c(500);
  *(undefined2 *)0x2074 = 0x3d;
  *(undefined1 *)0x799f = 10;
  FUN_1000_165a();
  while (cVar2 = FUN_184a_02fd(), cVar2 != '\0') {
    uVar3 = FUN_184a_030f();
    *(undefined1 *)0x2058 = uVar3;
  }
  local_4 = 0x3c;
  FUN_18ac_0000(0x1f,0x1f,0x1f,0xffff);
  uVar5 = *(int *)0x78c8 * 10;
  FUN_1000_146a(1,*(undefined2 *)0x78ce,1,0xb,0x19,0x1f,0x1b,(uint)*(byte *)0x79cc * 0xd00 + 0xc8a,
                unaff_DS,local_4,0x3c);
  local_4 = local_4 + 0x15;
  FUN_1920_154e(0xff,local_20a,unaff_SS,0,uVar5,0);
  uVar10 = 0x3c;
  puVar9 = local_318;
  uVar8 = unaff_SS;
  iVar4 = local_4;
  FUN_1920_09da((uint)*(byte *)0x79cc * 0xd00 + 0xb8a,unaff_DS);
  FUN_1920_0a67(local_20a,unaff_SS);
  FUN_1000_146a(1,*(undefined2 *)0x78ce,0x1b,9,0xfff1,0xfff4,0xfff0,puVar9,uVar8,iVar4,uVar10);
  local_4 = local_4 + 0x12;
  FUN_1000_146a(1,*(undefined2 *)0x78ce,0x1b,9,0x19,0xfff4,0xfff0,0x1d4f,0x1920,local_4,0x3c);
  local_4 = local_4 + 0x15;
  local_6 = 1;
  while( true ) {
    if (*(char *)(local_6 + 0x79e5) == '\x01') {
      if (local_6 == 1) {
        local_216 = (uint *)0x785a;
        local_8 = 0;
      }
      else {
        local_216 = (uint *)0x7888;
        local_8 = *(undefined2 *)0x2084;
      }
      _local_216 = (uint *)CONCAT22(unaff_DS,local_216);
      FUN_1920_154e(0xff,local_10a,unaff_SS,0,local_6,local_6 >> 0xf);
      puVar9 = local_318;
      uVar8 = unaff_SS;
      FUN_1920_09da((uint)*(byte *)0x79cc * 0xd00 + 0xa8a,unaff_DS);
      FUN_1920_0a67(0x1d5b,0x1920);
      FUN_1920_0a67(local_10a,unaff_SS);
      FUN_1920_0a67(0x1d5d,0x1920);
      FUN_1920_09f4(0xff,local_10a,unaff_SS,puVar9,uVar8);
      uVar6 = (uint)*(byte *)(local_6 * 4 + 0x1b69) * 100 +
              (uint)*(byte *)(local_6 * 4 + 0x1b6a) * 500 +
              (uint)*(byte *)(local_6 * 4 + 0x1b6b) * 2000;
      FUN_1920_154e(0xff,local_20a,unaff_SS,0,uVar6,(int)uVar6 >> 0xf);
      puVar9 = local_318;
      uVar8 = unaff_SS;
      FUN_1920_09da(local_10a,unaff_SS);
      FUN_1920_0a67(local_20a,unaff_SS);
      FUN_1920_09f4(0xff,local_10a,unaff_SS,puVar9,uVar8);
      FUN_1000_146a(1,*(undefined2 *)0x78ce,0x1b,9,0xd,0x1f,0x1b,local_10a,unaff_SS,local_4,0x3c);
      local_4 = local_4 + 0xb;
      local_218 = *(int *)(local_6 * 2 + 0x79f2) + -0x2400;
      uVar8 = (undefined2)((ulong)_local_216 >> 0x10);
      uVar7 = *_local_216;
      *_local_216 = *_local_216 + uVar6 + uVar5;
      puVar1 = (uint *)_local_216 + 1;
      *puVar1 = *puVar1 + ((int)uVar6 >> 0xf) + (uint)CARRY2(uVar6,uVar5) +
                (uint)CARRY2(uVar7,uVar6 + uVar5);
      FUN_1000_11b0((uint *)_local_216,uVar8);
      while ((byte)((uint *)_local_216)[0x16] < 2) {
        FUN_1000_1287(local_8);
        *(int *)0x2074 = local_218;
        FUN_1000_1239(local_8);
        FUN_184a_029c(0xf);
        uVar7 = FUN_1920_13a8(4);
        if (2 < uVar7) {
          *(undefined2 *)0x2074 = 0x21;
          FUN_1000_165a();
        }
      }
      FUN_184a_029c(200);
    }
    if (local_6 == 2) break;
    local_6 = local_6 + 1;
  }
  uVar3 = FUN_184a_030f();
  *(undefined1 *)0x2058 = uVar3;
  *(char *)0x79b7 = *(char *)0x79b7 + '\x01';
  local_6 = 1;
  while( true ) {
    if (*(char *)(local_6 + 0x79e5) == '\x02') {
      *(undefined1 *)(local_6 + 0x79e5) = 1;
    }
    if (local_6 == 2) break;
    local_6 = local_6 + 1;
  }
  return;
}



// ================================================
// Function: FUN_1000_2079 at 1000:2079
// ================================================

void FUN_1000_2079(void)

{
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  *(undefined2 *)0x2074 = 0x24;
  *(undefined1 *)0x799f = 2;
  FUN_1000_165a();
  return;
}



// ================================================
// Function: FUN_1000_20ac at 1000:20ac
// ================================================

void FUN_1000_20ac(void)

{
  undefined1 uVar1;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined1 *puVar2;
  undefined2 uVar3;
  int iVar4;
  undefined2 uVar5;
  undefined1 local_20a [256];
  undefined2 *local_10a;
  undefined1 local_106 [256];
  int local_6;
  int local_4;
  
  local_4 = 0x1000;
  local_6 = 0x20b7;
  FUN_1920_04df();
  FUN_1000_01fc();
  local_6 = 0x48;
  FUN_1000_146a(1,*(undefined2 *)0x78ce,0x1b,9,6,10,6,0x2095,0x1920,10,9);
  local_4 = 1;
  while( true ) {
    _local_10a = (undefined2 *)CONCAT22(unaff_DS,(undefined2 *)(local_4 * 0xd + 0x1af7));
    FUN_1920_154e(0xff,local_106,unaff_SS,0,*_local_10a,*(undefined2 *)(local_4 * 0xd + 0x1af9));
    uVar5 = 9;
    puVar2 = local_20a;
    uVar3 = unaff_SS;
    iVar4 = local_6;
    FUN_1920_09da((undefined2 *)_local_10a + 2,(int)((ulong)_local_10a >> 0x10));
    FUN_1920_0a67(0x20a7,0x1920);
    FUN_1920_0a67(local_106,unaff_SS);
    FUN_1000_146a(1,*(undefined2 *)0x78ce,0x1b,9,6,10,6,puVar2,uVar3,iVar4,uVar5);
    local_6 = local_6 + 0xb;
    if (local_4 == 7) break;
    local_4 = local_4 + 1;
  }
  uVar1 = FUN_184a_030f();
  *(undefined1 *)0x2058 = uVar1;
  return;
}



// ================================================
// Function: FUN_1000_216c at 1000:216c
// ================================================

void FUN_1000_216c(undefined2 param_1,undefined1 *param_2,int *param_3,byte *param_4,
                  undefined4 param_5)

{
  undefined2 uVar1;
  undefined2 unaff_SS;
  undefined1 *puVar2;
  undefined2 uVar3;
  undefined1 *puVar4;
  undefined2 uVar5;
  undefined1 local_304 [256];
  undefined1 local_204 [258];
  undefined1 local_102 [252];
  undefined2 uStack_6;
  undefined2 uStack_4;
  
  uStack_4 = 0x1000;
  uStack_6 = 0x2177;
  FUN_1920_04df();
  local_102[0] = 0;
  while ((uVar1 = (undefined2)((ulong)param_4 >> 0x10), ((byte *)param_4)[*param_3] != 0x5c &&
         (*param_3 <= (int)(uint)*param_4))) {
    puVar4 = local_304;
    uVar5 = unaff_SS;
    FUN_1920_09da(local_102,unaff_SS);
    puVar2 = local_204;
    uVar3 = unaff_SS;
    FUN_1920_0b0a(CONCAT11((char)((uint)*param_3 >> 8),((byte *)param_4)[*param_3]));
    FUN_1920_0a67(puVar2,uVar3);
    FUN_1920_09f4(0xff,local_102,unaff_SS,puVar4,uVar5);
    *param_3 = *param_3 + 1;
  }
  FUN_1920_09f4(0xff,(int)param_5,(int)((ulong)param_5 >> 0x10),local_102,unaff_SS);
  *param_3 = *param_3 + 1;
  if ((int)(uint)*param_4 < *param_3) {
    *param_2 = 1;
  }
  else {
    *param_2 = 0;
  }
  return;
}



// ================================================
// Function: FUN_1000_2226 at 1000:2226
// ================================================

void FUN_1000_2226(undefined2 param_1,char param_2,char param_3,int param_4,byte param_5,
                  byte param_6)

{
  undefined1 uVar1;
  int iVar2;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined1 *puVar3;
  undefined2 uVar4;
  undefined1 local_30a [256];
  char local_20a;
  undefined1 local_209;
  int local_208;
  undefined2 local_206 [2];
  undefined1 local_202 [256];
  undefined1 local_102 [252];
  undefined2 uStack_6;
  undefined2 uStack_4;
  
  uStack_4 = 0x1000;
  uStack_6 = 0x2231;
  FUN_1920_04df();
  local_209 = *(undefined1 *)0x78ce;
  if (param_3 != '\0') {
    FUN_1000_01fc();
  }
  local_208 = param_4;
  *(uint *)0x2082 = (uint)param_6;
  do {
    if ((uint)param_5 < *(uint *)0x2082) {
      if (param_3 != '\0') {
        uVar1 = FUN_184a_030f();
        *(undefined1 *)0x2058 = uVar1;
      }
      return;
    }
    iVar2 = (uint)*(byte *)0x79cc * 0xd00 + *(int *)0x2082 * 0x100 + -0x76;
    uVar1 = iVar2 == 0;
    FUN_1920_09f4(0xff,local_102,unaff_SS,iVar2,unaff_DS);
    local_206[0] = 1;
    while( true ) {
      puVar3 = local_30a;
      uVar4 = unaff_SS;
      FUN_1000_216c(param_1,&local_20a,unaff_SS,local_206,unaff_SS,local_102,unaff_SS);
      FUN_1920_09f4(0xff,local_202,unaff_SS,puVar3,uVar4);
      FUN_1920_0adf(0x2224,0x1920,local_202,unaff_SS);
      if ((bool)uVar1) {
        uVar1 = FUN_184a_030f();
        *(undefined1 *)0x2058 = uVar1;
        FUN_1000_01fc();
        local_208 = 0x14;
      }
      else {
        FUN_1000_146a(1,local_209,0x1b,9,6,10,6,local_202,unaff_SS,local_208,9);
        local_208 = local_208 + 10;
        if ((*(char *)0x79ba == '\x02') && (param_2 != '\0')) {
          local_209 = 0;
        }
        if ((*(char *)0x79ba == '\x01') && (param_3 != '\0')) {
          return;
        }
      }
      if (local_20a != '\0') break;
      uVar1 = 1;
    }
    *(int *)0x2082 = *(int *)0x2082 + 1;
  } while( true );
}



// ================================================
// Function: FUN_1000_2361 at 1000:2361
// ================================================

void __cdecl16near FUN_1000_2361(void)

{
  bool bVar1;
  char cVar2;
  undefined1 uVar3;
  int iVar4;
  undefined2 uVar5;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  if (*(char *)0x79f8 != '\x01') {
    FUN_18ac_05dc(0x234b,0x1920);
    iVar4 = FUN_1920_04a2();
    if (iVar4 != 0) {
      FUN_1000_00a3(1);
    }
    *(undefined1 *)0x79f8 = 1;
  }
  uVar5 = 0x1920;
  if (*(char *)0x208c == '\x01') {
    FUN_1000_2226(&stack0xfffe,0,1,0x14,3,1);
  }
LAB_1000_23a8:
  do {
    FUN_1000_030b(0x2355,uVar5);
    FUN_1000_2226(&stack0xfffe,1,0,0x4d,8,8);
    bVar1 = false;
    while (cVar2 = FUN_184a_02fd(), cVar2 != '\0') {
      uVar3 = FUN_184a_030f();
      *(undefined1 *)0x2058 = uVar3;
    }
    while( true ) {
      uVar5 = 0x184a;
      uVar3 = FUN_184a_030f();
      *(undefined1 *)0x2058 = uVar3;
      if (*(char *)0x2058 == '1') {
        FUN_1000_2079(&stack0xfffe);
        *(undefined1 *)0x79b8 = 1;
        bVar1 = true;
      }
      if (*(char *)0x2058 == '2') {
        FUN_1000_2079(&stack0xfffe);
        *(undefined1 *)0x79b8 = 2;
        bVar1 = true;
      }
      if (*(char *)0x2058 == 'i') {
        FUN_1000_2079(&stack0xfffe);
        FUN_1000_2226(&stack0xfffe,0,1,0x14,3,1);
        goto LAB_1000_23a8;
      }
      if (*(char *)0x2058 == 'z') {
        FUN_1000_2079(&stack0xfffe);
        FUN_1000_2226(&stack0xfffe,0,1,0x14,7,4);
        goto LAB_1000_23a8;
      }
      if (*(char *)0x2058 == 'r') {
        FUN_1000_2079(&stack0xfffe);
        FUN_1000_20ac(&stack0xfffe);
        goto LAB_1000_23a8;
      }
      if (*(char *)0x2058 == 'l') break;
      if (*(char *)0x2058 == '\x1b') {
        bVar1 = true;
        *(undefined1 *)0x79b8 = 0;
      }
      if (bVar1) {
        return;
      }
    }
    FUN_1000_2079(&stack0xfffe);
    *(byte *)0x79cc = *(byte *)0x79cc ^ 1;
  } while( true );
}



// ================================================
// Function: FUN_1000_247f at 1000:247f
// ================================================

void FUN_1000_247f(void)

{
  code *pcVar1;
  int iVar2;
  undefined2 uVar3;
  undefined1 uVar4;
  int in_CX;
  undefined2 extraout_DX;
  undefined2 extraout_DX_00;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  FUN_184a_029c(500);
  do {
    do {
      uVar4 = (undefined1)in_CX;
      pcVar1 = (code *)swi(0x21);
      (*pcVar1)();
      *(undefined1 *)0x79a8 = (char)((uint)extraout_DX >> 8);
      *(undefined1 *)0x79a9 = (char)extraout_DX;
      *(undefined1 *)0x79a4 = uVar4;
      FUN_184a_029c(1000);
      pcVar1 = (code *)swi(0x21);
      (*pcVar1)();
      *(undefined1 *)0x79ab = (char)((uint)extraout_DX_00 >> 8);
      *(undefined1 *)0x79ac = (char)extraout_DX_00;
      *(undefined1 *)0x79aa = uVar4;
      in_CX = ((uint)*(byte *)0x79ab - (uint)*(byte *)0x79a8) * 100;
      iVar2 = (((uint)*(byte *)0x79aa - (uint)*(byte *)0x79a4) * 6000 + in_CX +
              (uint)*(byte *)0x79ac) - (uint)*(byte *)0x79a9;
    } while (iVar2 < 1);
  } while (299 < iVar2);
  FUN_1920_0f0f();
  FUN_1920_0f01();
  uVar3 = FUN_1920_0f1b();
  *(undefined2 *)0x78ce = uVar3;
  FUN_1920_0f0f();
  FUN_1920_0f01();
  uVar3 = FUN_1920_0f1b();
  *(undefined2 *)0x78d0 = uVar3;
  FUN_1920_0f0f();
  FUN_1920_0f01();
  uVar3 = FUN_1920_0f1b();
  *(undefined2 *)0x78ca = uVar3;
  if (*(uint *)0x78ca < 2) {
    *(undefined2 *)0x78ca = 2;
  }
  return;
}



// ================================================
// Function: FUN_1000_2597 at 1000:2597
// ================================================

void FUN_1000_2597(void)

{
  int iVar1;
  undefined2 uVar2;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  byte local_b1;
  undefined1 local_b0 [128];
  undefined1 local_30 [42];
  undefined2 uStack_6;
  uint local_4;
  
  local_4 = 0x1000;
  uStack_6 = 0x25a2;
  FUN_1920_04df();
  *(undefined1 *)0x79c9 = *(undefined1 *)0x7f5c;
  *(undefined1 *)0x7f5c = 0;
  FUN_1920_142f();
  FUN_183f_0000(local_30,unaff_SS,0x20,0x2585,0x1920);
  *(undefined1 *)0x208c = 0;
  if (*(int *)0x7f5a == 0) {
    FUN_1920_15ca(0x2585,0x183f,local_b0,unaff_SS);
    FUN_1920_15f8(1,local_b0,unaff_SS);
    FUN_1920_16e3(0,0,1,&local_b1,unaff_SS,local_b0,unaff_SS);
    if (local_b1 != 0) {
      local_4 = 1;
      while( true ) {
        iVar1 = local_4 * 0xd;
        FUN_1920_16e3(0,0,4,iVar1 + 0x1af7,unaff_DS,local_b0,unaff_SS);
        FUN_1920_16e3(0,0,9,iVar1 + 0x1afb,unaff_DS,local_b0,unaff_SS);
        if (local_4 == local_b1) break;
        local_4 = local_4 + 1;
      }
    }
    FUN_1920_1679(local_b0,unaff_SS);
    iVar1 = FUN_1920_04a2();
    if (iVar1 != 0) {
      FUN_1000_00a3(1);
    }
  }
  else {
    local_4 = 1;
    uVar2 = 0x183f;
    while( true ) {
      iVar1 = local_4 * 0xd;
      FUN_1920_09f4(8,iVar1 + 0x1afb,unaff_DS,0x258e,uVar2);
      *(undefined2 *)CONCAT22(unaff_DS,(undefined2 *)(iVar1 + 0x1af7)) = 10000;
      *(undefined2 *)(iVar1 + 0x1af9) = 0;
      if (local_4 == 7) break;
      local_4 = local_4 + 1;
      uVar2 = 0x1920;
    }
    FUN_1000_16ca();
  }
  return;
}



// ================================================
// Function: FUN_1000_26e8 at 1000:26e8
// ================================================

void __cdecl16near FUN_1000_26e8(void)

{
  undefined2 uVar1;
  uint uVar2;
  int iVar3;
  undefined2 uVar4;
  int extraout_DX;
  int extraout_DX_00;
  int extraout_DX_01;
  int extraout_DX_02;
  int extraout_DX_03;
  undefined2 in_BX;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined4 uVar5;
  int local_4;
  
  local_4 = 0x1000;
  FUN_1920_04df();
  FUN_1000_247f(&stack0xfffe);
  *(undefined1 *)0xc496 = 0;
  *(undefined1 *)0x79f9 = 0;
  out(0x60,0xf3);
  FUN_184a_029c(200);
  out(0x60,0x67);
  *(undefined2 *)0x2082 = 0;
  while( true ) {
    uVar4 = 0;
    FUN_1920_0f0f();
    FUN_1920_0efb();
    FUN_1920_0f01();
    uVar1 = FUN_1920_1034();
    iVar3 = *(int *)0x2082 * 6;
    *(undefined2 *)(iVar3 + 0x7bda) = uVar1;
    *(undefined2 *)(iVar3 + 0x7bdc) = in_BX;
    *(undefined2 *)(iVar3 + 0x7bde) = uVar4;
    if (*(int *)0x2082 == 0x7f) break;
    *(int *)0x2082 = *(int *)0x2082 + 1;
  }
  *(undefined1 *)0x79cc = 0;
  *(undefined1 *)0x79c4 = 0;
  *(undefined1 *)0x79f8 = 0;
  *(undefined2 *)0x2084 = 0x2e;
  FUN_183f_007b(0x1b60,unaff_DS,9);
  FUN_183f_0093(0x1091,0x1000,9);
  FUN_183f_007b(0x1b64,unaff_DS,0x1c);
  FUN_183f_0093(0xfbe,0x1000,0x1c);
  iVar3 = extraout_DX;
  uVar2 = FUN_1920_02b8();
  if ((iVar3 < 0) || ((iVar3 < 1 && (uVar2 < 4)))) {
    FUN_1000_00a3(2);
    iVar3 = extraout_DX_00;
  }
  else {
    uVar5 = FUN_1920_023f(0x318);
    iVar3 = (int)((ulong)uVar5 >> 0x10);
    *(undefined2 *)0x79c0 = (int)uVar5;
    *(int *)0x79c2 = iVar3;
  }
  uVar2 = FUN_1920_02b8();
  if ((iVar3 < 0) || ((iVar3 < 1 && (uVar2 < 57000)))) {
    FUN_1000_00a3(2);
    iVar3 = extraout_DX_01;
  }
  else {
    uVar5 = FUN_1920_023f(57000);
    iVar3 = (int)((ulong)uVar5 >> 0x10);
    *(undefined2 *)0x7f70 = (int)uVar5;
    *(int *)0x7f72 = iVar3;
  }
  *(undefined2 *)0xc212 = (int)((ulong)*(undefined4 *)0x7f70 >> 0x10);
  *(undefined2 *)0xc214 = (int)*(undefined4 *)0x7f70;
  *(undefined2 *)0xc208 = 0x7f74;
  uVar2 = FUN_1920_02b8();
  if ((iVar3 < 0) || ((iVar3 < 1 && (uVar2 < 0x5334)))) {
    FUN_1000_00a3(2);
    iVar3 = extraout_DX_02;
  }
  else {
    uVar5 = FUN_1920_023f(0x5334);
    iVar3 = (int)((ulong)uVar5 >> 0x10);
    *(undefined2 *)0xc1e4 = (int)uVar5;
    *(int *)0xc1e6 = iVar3;
  }
  *(undefined2 *)0xc204 = 0x28;
  *(undefined2 *)0xc1ee = 0xa0;
  *(undefined2 *)0xc1fc = 0xc21e;
  *(undefined2 *)0xc1fa = (int)((ulong)*(undefined4 *)0xc1e4 >> 0x10);
  uVar2 = FUN_1920_02b8();
  if ((iVar3 < 0) || ((iVar3 < 1 && (uVar2 < 60000)))) {
    FUN_1000_00a3(2);
    iVar3 = extraout_DX_03;
  }
  else {
    uVar5 = FUN_1920_023f(60000);
    iVar3 = (int)((ulong)uVar5 >> 0x10);
    *(undefined2 *)0xc498 = (int)uVar5;
    *(int *)0xc49a = iVar3;
  }
  uVar2 = FUN_1920_02b8();
  if ((iVar3 < 0) || ((iVar3 < 1 && (uVar2 < 0x8728)))) {
    FUN_1000_00a3(2);
  }
  local_4 = 0;
  FUN_1000_0000(0x1a00,&local_4,unaff_SS,0x8a,unaff_DS);
  if (local_4 != -0x2c9) {
    FUN_183f_0093(*(undefined2 *)0x1b60,*(undefined2 *)0x1b62,9);
    FUN_183f_0093(*(undefined2 *)0x1b64,*(undefined2 *)0x1b66,0x1c);
    FUN_1920_00e9();
  }
  FUN_18ac_0422(0x26d5,0x1920);
  iVar3 = FUN_1920_04a2();
  if (iVar3 != 0) {
    FUN_1000_00a3(1);
  }
  FUN_1000_0630();
  *(undefined1 *)0x79a0 = 0;
  *(undefined1 *)0x79a2 = 1;
  *(undefined2 *)0x79ee = 0x28;
  FUN_18ac_0417();
  *(int *)0x2070 = *(int *)0xc208 + 0x1e40;
  if (*(char *)0x79f8 != '\x01') {
    FUN_18ac_05dc(0x26de,0x18ac);
    iVar3 = FUN_1920_04a2();
    if (iVar3 != 0) {
      FUN_1000_00a3(1);
    }
    *(undefined1 *)0x79f8 = 1;
  }
  *(undefined2 *)0x207c = 0x209e;
  *(undefined2 *)0x207a = 0x6620;
  FUN_1000_2597(&stack0xfffe);
  FUN_1000_247f(&stack0xfffe);
  *(undefined2 *)0x78cc = *(undefined2 *)0x78d0;
  return;
}



// ================================================
// Function: FUN_1000_2959 at 1000:2959
// ================================================

void __cdecl16near FUN_1000_2959(void)

{
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  if (*(char *)0x79b8 == '\x01') {
    FUN_1000_07fa(0x1c,0xf,0xa0,*(int *)0x79ee << 3,0,*(int *)0x79ee * -4 + 0xa0);
    *(int *)0x79f0 = *(int *)0x79ee * -4 + 0x5a4;
    *(undefined2 *)0x79f2 = *(undefined2 *)0x79f0;
  }
  else if (*(char *)0x79b8 == '\x02') {
    FUN_1000_07fa(0x1c,0xf,0xa0,0xa0,0,0);
    FUN_1000_07fa(0xc,4,0xa0,0xa0,0,0xa0);
    *(undefined2 *)0x79f0 = 0x504;
    *(undefined2 *)0x79f2 = 0x5a4;
  }
  return;
}



// ================================================
// Function: FUN_1000_29db at 1000:29db
// ================================================

void FUN_1000_29db(int param_1)

{
  byte bVar1;
  int iVar2;
  undefined2 unaff_DS;
  undefined2 local_4;
  
  FUN_1920_04df();
  bVar1 = *(byte *)(param_1 + 0x79e9);
  iVar2 = *(int *)(param_1 * 2 + 0x79f2);
  *(int *)0x2074 = *(int *)0xc208 + 0x1cc0;
  *(int *)0x2072 = iVar2 + -0x1000;
  local_4 = 1;
  while( true ) {
    if ((int)(uint)bVar1 < local_4) {
      *(undefined2 *)0x2074 = *(undefined2 *)0xc208;
    }
    FUN_1000_1156();
    *(int *)0x2072 = *(int *)0x2072 + 9;
    if (local_4 == 3) break;
    local_4 = local_4 + 1;
  }
  return;
}



// ================================================
// Function: FUN_1000_2a41 at 1000:2a41
// ================================================

void FUN_1000_2a41(undefined2 param_1,byte param_2)

{
  FUN_1920_04df();
  FUN_1000_07fa(0xffef,3,0x11,0x58,0xac,param_2);
  FUN_1000_07c5(0x1c,3,0x66,0xa4,param_2);
  FUN_1000_07fa(0x19,0x1b,0x14,0x14,0xa1,param_2 + 0x77);
  FUN_1000_07c5(0xffef,9,0x14,0xb5,param_2 + 0x77);
  return;
}



// ================================================
// Function: FUN_1000_2adc at 1000:2adc
// ================================================

void __cdecl16near FUN_1000_2adc(void)

{
  code *pcVar1;
  undefined1 uVar2;
  int iVar3;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined1 **ppuVar4;
  undefined2 uVar5;
  undefined1 *local_202;
  undefined1 local_102 [252];
  undefined2 uStack_6;
  undefined2 uStack_4;
  
  uStack_4 = 0x1000;
  uStack_6 = 0x2ae7;
  FUN_1920_04df();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  *(undefined1 *)0x1b74 = 1;
  *(undefined1 *)0x1b75 = 1;
  FUN_1000_0faa();
  *(undefined1 *)0x79b6 = 200;
  *(undefined2 *)0x208a = 20000;
  *(undefined1 *)0x79ca = 1;
  local_202 = (undefined1 *)(uint)*(byte *)0x79b7;
  FUN_1920_154e(0xff,local_102,unaff_SS,0);
  *(undefined2 *)0x206e = (int)((ulong)*(undefined4 *)0x6612 >> 0x10);
  *(undefined1 *)0x79c5 = 0;
  *(undefined1 *)0x79c6 = 0;
  *(undefined2 *)0xc20a = 0;
  *(undefined2 *)0xc20c = 0;
  *(undefined2 *)0xc1f0 = (int)*(undefined4 *)0xc1e0;
  *(undefined1 *)0x1b78 = 0;
  *(undefined1 *)0x1b7a = 0;
  *(undefined1 *)0x1b79 = 0;
  *(undefined1 *)0x1b7b = 0;
  *(undefined2 *)0x2098 = 0;
  *(undefined1 *)0x1b7e = 0;
  *(undefined1 *)0x1b7f = 0;
  *(undefined1 *)0x1b80 = 0;
  *(undefined1 *)0x1b7d = 0;
  *(undefined1 *)0x1b7c = 0;
  *(undefined1 *)0x1b81 = 0;
  *(undefined2 *)0x79f6 = 0xb4;
  *(undefined2 *)0x79f4 = 0;
  *(undefined2 *)0x209c = 0;
  *(undefined2 *)0x209a = 0;
  *(undefined1 *)0x79b0 = 0;
  *(undefined1 *)0x79b1 = 0;
  *(undefined1 *)0x208d = 0;
  *(undefined2 *)0x207e = 199;
  *(undefined2 *)0x2080 = 0;
  *(undefined2 *)0x2076 = 0;
  *(undefined1 *)0x208e = 0;
  *(undefined2 *)0x78c8 = 0;
  *(undefined1 *)0x79b2 = 0;
  *(undefined2 *)0x2088 = 0;
  *(undefined1 *)0x79b5 = 0;
  *(undefined1 *)0x79cb = 0;
  *(undefined1 *)0x79f9 = 0;
  *(int *)0x2066 = *(int *)0xc204 << 1;
  if (*(char *)0x79b8 == '\x02') {
    local_202 = (undefined1 *)0x2bf0;
    FUN_1000_0838();
  }
  else {
    local_202 = (undefined1 *)0x2bf9;
    FUN_1000_0838();
  }
  local_202 = (undefined1 *)0x2aa7;
  FUN_1000_0709();
  FUN_1000_01fc();
  *(undefined2 *)0x2074 = 0x78;
  *(undefined1 *)0x799f = 0xb;
  FUN_1000_165a();
  if (*(char *)0x79f8 != '\x01') {
    local_202 = (undefined1 *)0x2ab2;
    FUN_18ac_05dc();
    local_202 = (undefined1 *)0x2c28;
    iVar3 = FUN_1920_04a2();
    if (iVar3 != 0) {
      local_202 = (undefined1 *)0x2c31;
      FUN_1000_00a3();
    }
    *(undefined1 *)0x79f8 = 1;
  }
  local_202 = (undefined1 *)0x5e;
  ppuVar4 = &local_202;
  uVar5 = unaff_SS;
  FUN_1920_09da((uint)*(byte *)0x79cc * 0xd00 + 0x88a,unaff_DS);
  FUN_1920_0a67(local_102,unaff_SS);
  FUN_1000_146a(1,*(undefined2 *)0x78ce,0x1b,0xb,1,0x1f,0x1b,ppuVar4,uVar5);
  local_202 = (undefined1 *)0x2c77;
  uVar2 = FUN_184a_030f();
  *(undefined1 *)0x2058 = uVar2;
  local_202 = (undefined1 *)0x0;
  FUN_1000_07c5(0,200,0x140);
  if (*(char *)0x79f8 != '\x02') {
    if (*(char *)0x79b7 == '\a') {
      local_202 = (undefined1 *)0x2ac9;
      FUN_18ac_05dc();
    }
    else {
      local_202 = (undefined1 *)0x2abc;
      FUN_18ac_05dc();
    }
    local_202 = (undefined1 *)0x2cb2;
    iVar3 = FUN_1920_04a2();
    if (iVar3 != 0) {
      local_202 = (undefined1 *)0x2cbb;
      FUN_1000_00a3();
    }
    *(undefined1 *)0x79f8 = 2;
  }
  *(undefined2 *)0x2082 = 1;
  while( true ) {
    if (*(char *)(*(int *)0x2082 + 0x79e5) == '\x01') {
      local_202 = &stack0xfffe;
      FUN_1000_2a41();
    }
    *(undefined1 *)(*(int *)0x2082 + 0x79eb) = 0xff;
    if (*(int *)0x2082 == 2) break;
    *(int *)0x2082 = *(int *)0x2082 + 1;
  }
  FUN_1000_2959();
  local_202 = (undefined1 *)0xa0;
  FUN_1000_07fa(0xffe0,3,0x27,0x25);
  local_202 = (undefined1 *)0xa5;
  FUN_1000_07c5(0xfff5,10,10);
  local_202 = (undefined1 *)0xb5;
  FUN_1000_07c5(0xfff6,10,10);
  local_202 = (undefined1 *)0x2d2e;
  FUN_1000_1174();
  if (*(char *)0x79e6 == '\x01') {
    *(int *)0x2074 = *(int *)0x79f4 + -0x2400;
    local_202 = (undefined1 *)0x2d43;
    FUN_1000_1239();
  }
  if (*(char *)0x79e7 == '\x01') {
    *(int *)0x2074 = *(int *)0x79f6 + -0x2400;
    local_202 = (undefined1 *)0x2d5a;
    FUN_1000_1239();
  }
  *(undefined1 *)0xc496 = 1;
  *(undefined1 *)0xc222 = *(undefined1 *)0xc326;
  *(undefined1 *)0xc223 = *(undefined1 *)0xc327;
  *(undefined2 *)0xc224 = *(undefined2 *)0xc328;
  *(undefined2 *)0xc216 = 0;
  *(undefined2 *)0xc218 = 0;
  *(char *)0xc496 = *(char *)0xc496 + '\x01';
  *(undefined1 *)0xc22a = *(undefined1 *)0xc326;
  *(undefined1 *)0xc22b = *(undefined1 *)0xc327;
  *(undefined2 *)0xc22c = *(undefined2 *)0xc328;
  *(undefined1 *)0x1b89 = 0;
  *(undefined2 *)0x1b8e = 0;
  *(undefined2 *)0x1b90 = 0;
  *(undefined2 *)0x1b92 = 0;
  *(undefined2 *)0x1b94 = 0;
  *(undefined1 *)0x1b9d = 0;
  *(undefined1 *)0x1b88 = 0;
  *(undefined1 *)0x1b9c = 0;
  *(undefined2 *)0x1b96 = 0;
  local_202 = (undefined1 *)0x1b9e;
  FUN_1000_06ab(1,1,9,2);
  *(undefined1 *)0x1baf = 1;
  *(undefined2 *)0x1bb4 = 0;
  *(undefined2 *)0x1bb6 = 0;
  *(undefined2 *)0x1bb8 = 0;
  *(undefined2 *)0x1bba = 0;
  *(undefined1 *)0x1bc3 = 1;
  *(undefined1 *)0x1bae = 0;
  *(undefined1 *)0x1bc2 = 0;
  *(undefined2 *)0x1bbc = 0;
  local_202 = (undefined1 *)0x1bc4;
  FUN_1000_06ab(1,1,0x1c,0x15);
  if (*(char *)0x79e6 == '\x01') {
    local_202 = (undefined1 *)0x1b88;
    FUN_1000_056b(1);
  }
  else {
    *(undefined2 *)0xc21e = 10000;
    *(undefined2 *)0xc220 = 6000;
  }
  if (*(char *)0x79e7 == '\x01') {
    local_202 = (undefined1 *)0x1bae;
    FUN_1000_056b(2);
  }
  else {
    *(undefined2 *)0xc226 = 10000;
    *(undefined2 *)0xc228 = 6000;
  }
  *(undefined1 *)0x1b76 = 2;
  *(undefined1 *)0x1b77 = 2;
  *(undefined2 *)0x2082 = 1;
  while( true ) {
    if (*(char *)(*(int *)0x2082 + 0x79e5) == '\x01') {
      local_202 = (undefined1 *)0x2e71;
      FUN_1000_29db();
    }
    if (*(int *)0x2082 == 2) break;
    *(int *)0x2082 = *(int *)0x2082 + 1;
  }
  if (*(char *)0x79b7 == '\a') {
    local_202 = (undefined1 *)0x2ad3;
    FUN_1000_08a5(100,100);
  }
  return;
}



// ================================================
// Function: FUN_1000_2e8d at 1000:2e8d
// ================================================

void FUN_1000_2e8d(undefined2 param_1,undefined2 *param_2)

{
  undefined2 *puVar1;
  undefined2 uVar2;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  uVar2 = (undefined2)((ulong)param_2 >> 0x10);
  puVar1 = (undefined2 *)param_2;
  *(undefined2 *)0x2082 = 0;
  while( true ) {
    puVar1[*(int *)0x2082 + 2] = *(undefined2 *)0x2070;
    puVar1[*(int *)0x2082 + 0xc] = *(undefined2 *)0x2070;
    if (*(int *)0x2082 == 9) break;
    *(int *)0x2082 = *(int *)0x2082 + 1;
  }
  *(undefined1 *)(puVar1 + 0x16) = 2;
  *param_2 = 0;
  puVar1[1] = 0;
  return;
}



// ================================================
// Function: FUN_1000_2efd at 1000:2efd
// ================================================

void __cdecl16near FUN_1000_2efd(void)

{
  undefined2 uVar1;
  int iVar2;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  FUN_1920_15ca(0x2ef2,0x1920,0x7eda,unaff_DS);
  FUN_1920_15f8(1,0x7eda,unaff_DS);
  iVar2 = FUN_1920_04a2();
  if (iVar2 != 0) {
    FUN_1000_00a3(1);
  }
  *(undefined1 *)0x1bac = 100;
  *(undefined1 *)0x1bd2 = 100;
  *(undefined1 *)0x79c7 = 0;
  *(undefined1 *)0x79e6 = 1;
  *(undefined1 *)0x79e7 = 1;
  *(undefined1 *)0xc49c = 1;
  if (*(char *)0x79b8 == '\x01') {
    *(undefined1 *)0x79e7 = 0;
  }
  *(undefined1 *)0x79b7 = 1;
  *(undefined1 *)0x79ea = 2;
  *(undefined1 *)0x79eb = 2;
  *(undefined1 *)0x1b6c = 200;
  *(undefined1 *)0x1b6d = 0x14;
  *(undefined1 *)0x1b6e = 6;
  *(undefined1 *)0x1b6f = 0;
  uVar1 = *(undefined2 *)0x1b6e;
  *(undefined2 *)0x1b70 = *(undefined2 *)0x1b6c;
  *(undefined2 *)0x1b72 = uVar1;
  FUN_1000_2e8d(&stack0xfffe,0x785a,unaff_DS);
  FUN_1000_2e8d(&stack0xfffe,0x7888,unaff_DS);
  return;
}



// ================================================
// Function: FUN_1000_2f9f at 1000:2f9f
// ================================================

void FUN_1000_2f9f(undefined1 param_1,undefined1 param_2,undefined1 param_3,int param_4,uint param_5
                  ,uint param_6,undefined2 param_7,undefined2 param_8)

{
  uint uVar1;
  int iVar2;
  undefined2 unaff_DS;
  undefined2 local_6;
  
  FUN_1920_04df();
  if (*(byte *)0x208d < 0x1e) {
    *(char *)0x208d = *(char *)0x208d + '\x01';
    *(undefined2 *)0x2072 = 1;
    uVar1 = (int)param_6 >> 0xf;
    if (0x7ff < (int)((param_6 ^ uVar1) - uVar1)) {
      param_6 = ((int)((param_6 ^ uVar1) - uVar1) / (int)param_6) * 0x7ff;
    }
    uVar1 = (int)param_5 >> 0xf;
    if (0x7ff < (int)((param_5 ^ uVar1) - uVar1)) {
      param_5 = ((int)((param_5 ^ uVar1) - uVar1) / (int)param_5) * 0x7ff;
    }
    iVar2 = (uint)*(byte *)0x208d * 0x26;
    _local_6 = (undefined1 *)CONCAT22(unaff_DS,(undefined1 *)(iVar2 + 0x1bae));
    *(undefined2 *)(iVar2 + 0x1bb8) = 0;
    *(undefined2 *)(iVar2 + 0x1bba) = 0;
    *(undefined1 *)(iVar2 + 0x1baf) = *(undefined1 *)0xc496;
    *(uint *)(iVar2 + 0x1bb4) = param_6;
    *(uint *)(iVar2 + 0x1bb6) = param_5;
    *_local_6 = param_3;
    *(undefined1 *)(iVar2 + 0x1bb0) = param_2;
    *(undefined1 *)(iVar2 + 0x1bc3) = param_1;
    if (param_4 == 0x1f) {
      *(undefined1 *)(iVar2 + 0x1bc2) = 0;
    }
    else {
      *(char *)(iVar2 + 0x1bc2) = '\x10' - *(char *)(param_4 * 4 + -0x3cdd);
    }
    FUN_18ac_0517(param_7,param_8,param_4);
  }
  else {
    *(undefined2 *)0x2072 = 0;
  }
  return;
}



// ================================================
// Function: FUN_1000_30a3 at 1000:30a3
// ================================================

void FUN_1000_30a3(byte param_1)

{
  char *pcVar1;
  int iVar2;
  int iVar3;
  char *pcVar4;
  undefined2 unaff_DS;
  undefined4 uVar5;
  
  uVar5 = 0x100030ae;
  FUN_1920_04df();
  *(undefined2 *)0x2074 = 0x56;
  *(undefined1 *)0x799f = 5;
  FUN_1000_165a(uVar5);
  pcVar4 = (char *)0x0;
  iVar2 = *(int *)0x78ba;
  iVar3 = 0;
  do {
    pcVar1 = pcVar4;
    pcVar4 = pcVar4 + 1;
    if (*pcVar1 == *(char *)0x79b4) {
      iVar3 = iVar3 + 1;
    }
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  *(int *)0x2074 = iVar3;
  if ((uint)(*(int *)0x2088 + *(int *)0x2074) < *(uint *)0x2086) {
    *(undefined1 *)0x79ca = 0;
  }
  else {
    *(undefined1 *)0x79ca = 1;
  }
  iVar2 = (uint)param_1 * 0x26 + 0x1b62;
  FUN_1000_06ab(1,3,*(undefined1 *)0x6d,*(undefined1 *)0x6c,(uint)param_1 * 0x26 + 0x1b78,unaff_DS);
  *(undefined1 *)(iVar2 + 0x15) = 2;
  *(undefined2 *)(iVar2 + 0x10) = 0x3c;
  *(undefined1 *)(iVar2 + 0x24) = 100;
  return;
}



// ================================================
// Function: FUN_1000_313d at 1000:313d
// ================================================

void FUN_1000_313d(int param_1,uint param_2)

{
  undefined2 unaff_DS;
  undefined2 uVar1;
  
  uVar1 = 0x1000;
  FUN_1920_04df();
  for (; param_1 != 0; param_1 = param_1 + -1) {
    *(int *)0x2074 = (param_2 % 10) * 0x40 + *(int *)0x2070;
    FUN_1000_1156(uVar1);
    *(int *)0x2072 = *(int *)0x2072 + -9;
    param_2 = param_2 / 10;
  }
  return;
}



// ================================================
// Function: FUN_1000_3184 at 1000:3184
// ================================================

void FUN_1000_3184(int param_1)

{
  undefined2 unaff_DS;
  undefined2 local_4;
  
  FUN_1920_04df();
  *(int *)0x2072 = param_1 + 0x519;
  if (*(int *)0x208a != *(int *)0x2088) {
    *(undefined2 *)0x208a = *(undefined2 *)0x2088;
    FUN_1000_0495(0x1aae,unaff_DS,0x1ab2,unaff_DS,0xfff5);
    local_4 = *(int *)0x2086 - *(int *)0x2088;
    if (local_4 < 1) {
      local_4 = 0;
      *(undefined1 *)0x79c5 = 1;
    }
    FUN_1000_313d(2,local_4);
  }
  if (*(char *)0x79b6 != *(char *)0x79b5) {
    FUN_1000_0495(0x1aae,unaff_DS,0x1ab2,unaff_DS,0xfff6);
    *(undefined1 *)0x79b6 = *(undefined1 *)0x79b5;
    *(int *)0x2072 = param_1 + 0x1919;
    if ((uint)*(byte *)0x79b5 < (uint)*(byte *)0x79b3) {
      local_4 = (uint)*(byte *)0x79b3 - (uint)*(byte *)0x79b5;
    }
    else {
      local_4 = 0;
      *(undefined1 *)0x79c6 = 1;
    }
    FUN_1000_313d(2,local_4);
  }
  if ((*(char *)0x79c5 != '\0') && (*(char *)0x79c6 != '\0')) {
    FUN_1000_0495(0x1aae,unaff_DS,0x1ab2,unaff_DS,0xffe0);
  }
  return;
}



// ================================================
// Function: FUN_1000_326e at 1000:326e
// ================================================

void FUN_1000_326e(byte param_1,int param_2,int param_3)

{
  byte bVar1;
  int iVar2;
  undefined2 unaff_DS;
  undefined2 local_c;
  undefined1 local_4;
  
  FUN_1920_04df();
  bVar1 = *(byte *)(param_1 + 0x1b73);
  local_4 = *(byte *)((uint)param_1 * 4 + (uint)bVar1 + 0x1b67);
  if (1 < *(byte *)(param_1 + 0x1b75)) {
    FUN_1000_07c5(0x12,0x10,0x10,param_2,param_3);
    *(undefined1 *)0x79a3 = 0x12;
    iVar2 = (bVar1 + 0x39) * 4;
    _local_c = (byte *)CONCAT22(unaff_DS,(byte *)(iVar2 + -0x3cde));
    FUN_1000_12db(bVar1 + 0x39,(0x10 - *(byte *)(iVar2 + -0x3cdd) >> 1) + param_2,
                  (0x10 - *_local_c >> 1) + param_3);
  }
  *(undefined1 *)(param_1 + 0x1b75) = 0;
  if (99 < local_4) {
    local_4 = 99;
  }
  *(int *)0x2072 = (param_2 + 0x13) * 0x140 + param_3 + 8;
  FUN_1000_313d(2,local_4);
  return;
}



// ================================================
// Function: FUN_1000_3358 at 1000:3358
// ================================================

void FUN_1000_3358(int param_1)

{
  char *pcVar1;
  byte bVar2;
  byte bVar3;
  int iVar4;
  undefined2 unaff_DS;
  byte *local_c;
  uint local_4;
  
  FUN_1920_04df();
  if (param_1 <= (int)(uint)*(byte *)0x208d) {
    bVar3 = *(byte *)(param_1 * 0x26 + 0x1baf);
    FUN_18ac_0594(bVar3);
    bVar2 = *(byte *)0x208d;
    if (param_1 <= (int)(bVar2 - 1)) {
      for (local_4 = param_1;
          FUN_1920_090e(0x26,local_4 * 0x26 + 0x1bae,unaff_DS,(local_4 + 1) * 0x26 + 0x1bae,unaff_DS
                       ), local_4 != bVar2 - 1; local_4 = local_4 + 1) {
      }
    }
    *(char *)0x208d = *(char *)0x208d + -1;
    bVar2 = *(byte *)0x208d;
    if (bVar2 < 0x8000) {
      local_4 = 0;
      while( true ) {
        if (bVar3 < *(byte *)(local_4 * 0x26 + 0x1baf)) {
          pcVar1 = (char *)(local_4 * 0x26 + 0x1baf);
          *pcVar1 = *pcVar1 + -1;
        }
        if (local_4 == bVar2) break;
        local_4 = local_4 + 1;
      }
    }
    if ((*(char *)0x79f9 != '\0') && (bVar2 = *(byte *)0x79f9, bVar2 != 0)) {
      local_4 = 1;
      while( true ) {
        iVar4 = local_4 * 0x10;
        _local_c = (byte *)CONCAT22(unaff_DS,(byte *)(iVar4 + 0x79ea));
        if (bVar3 < *_local_c) {
          *_local_c = *_local_c - 1;
        }
        if (bVar3 < *(byte *)(iVar4 + 0x79eb)) {
          *(char *)(iVar4 + 0x79eb) = *(char *)(iVar4 + 0x79eb) + -1;
        }
        if (local_4 == bVar2) break;
        local_4 = local_4 + 1;
      }
    }
  }
  return;
}



// ================================================
// Function: FUN_1000_346b at 1000:346b
// ================================================

void FUN_1000_346b(undefined2 *param_1,undefined2 *param_2,undefined2 param_3,undefined2 param_4,
                  int *param_5,int *param_6)

{
  undefined2 uVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  int iVar4;
  int iVar5;
  undefined2 in_BX;
  bool bVar6;
  
  FUN_1920_04df();
  iVar4 = *param_6 >> 0xf;
  uVar1 = FUN_1920_0f0f();
  iVar5 = *param_5 >> 0xf;
  uVar3 = in_BX;
  uVar2 = FUN_1920_0f0f();
  bVar6 = true;
  FUN_1920_0f0b(uVar2,uVar3,iVar5,uVar1);
  if (bVar6) {
    bVar6 = true;
    FUN_1920_0f0b(uVar2,uVar3,iVar5,uVar1);
    if (bVar6) goto LAB_1000_353f;
  }
  uVar1 = FUN_1920_0efb();
  FUN_1920_0efb(uVar1,uVar3,iVar5);
  FUN_1920_0ee9();
  uVar3 = FUN_1920_0fc2();
  FUN_1920_0f0f(uVar3,in_BX,iVar4);
  FUN_1920_0f01();
LAB_1000_353f:
  FUN_1920_0efb();
  uVar3 = FUN_1920_0f13();
  *param_2 = uVar3;
  FUN_1920_0efb();
  uVar3 = FUN_1920_0f13();
  *param_1 = uVar3;
  return;
}



// ================================================
// Function: FUN_1000_3587 at 1000:3587
// ================================================

void FUN_1000_3587(int param_1,int param_2)

{
  int iVar1;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  *(uint *)0xc216 = param_2 - *(int *)0x78bc & 0xfff8;
  *(uint *)0xc218 = param_1 - 0x50U & 0xfff8;
  iVar1 = FUN_1920_092a();
  *(int *)0xc1f0 = (param_2 / 8 - *(int *)0x78be) + iVar1;
  *(int *)0xc20a = param_2 % 8;
  *(int *)0xc20c = param_1 % 8;
  if (*(int *)0xc216 < 0) {
    *(int *)0xc1f0 = *(int *)0xc1f0 - *(int *)0xc216 / 8;
    *(undefined2 *)0xc20a = 0;
    *(undefined2 *)0xc216 = 0;
  }
  else if (*(int *)0x2094 < *(int *)0xc216) {
    *(int *)0xc1f0 = *(int *)0xc1f0 + (*(int *)0x2094 - *(int *)0xc216) / 8;
    *(undefined2 *)0xc20a = 7;
    *(undefined2 *)0xc216 = *(undefined2 *)0x2094;
  }
  if (*(int *)0xc218 < 0) {
    iVar1 = FUN_1920_092a();
    *(int *)0xc1f0 = *(int *)0xc1f0 - iVar1;
    *(undefined2 *)0xc20c = 0;
    *(undefined2 *)0xc218 = 0;
  }
  else if (*(int *)0x2096 < *(int *)0xc218) {
    iVar1 = FUN_1920_092a();
    *(int *)0xc1f0 = *(int *)0xc1f0 - iVar1;
    *(undefined2 *)0xc20c = 7;
    *(undefined2 *)0xc218 = *(undefined2 *)0x2096;
  }
  *(undefined2 *)0xc1f0 = *(undefined2 *)0xc1f0;
  *(int *)0xc20a = *(int *)0xc20a + *(int *)0x2098;
  return;
}



// ================================================
// Function: FUN_1000_370e at 1000:370e
// ================================================

void FUN_1000_370e(undefined1 *param_1,char param_2,char param_3,int param_4)

{
  uint uVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  bool bVar5;
  bool bVar6;
  char cVar7;
  int iVar8;
  int iVar9;
  uint uVar10;
  uint uVar11;
  int iVar12;
  uint *puVar13;
  uint *puVar14;
  undefined2 uVar15;
  undefined2 unaff_DS;
  int *local_20;
  int local_12;
  int local_10;
  uint *local_e;
  uint *local_c;
  int local_a;
  uint *local_8;
  
  FUN_1920_04df();
  uVar1 = *(uint *)((int)*(undefined4 *)0x6612 + param_4 * 2);
  iVar12 = *(int *)0xc204;
  if ((uVar1 & 0x8000) == 0) {
    if (uVar1 < 0x4000) {
      local_a = param_4 * 2;
      local_c = (uint *)(local_a + iVar12 * -2);
      local_8 = local_c + -1;
      local_e = (uint *)(local_a + -2);
      *param_1 = 0;
      local_10 = 2;
      local_12 = 2;
      do {
        bVar6 = true;
        bVar2 = false;
        bVar3 = false;
        bVar4 = false;
        bVar5 = false;
        uVar15 = (undefined2)((ulong)*(undefined4 *)0x6612 >> 0x10);
        iVar8 = local_10;
        puVar14 = local_8;
        do {
          iVar9 = local_12;
          puVar13 = local_8;
          if (*puVar14 == uVar1) {
            bVar6 = false;
            bVar2 = true;
            uVar15 = (undefined2)((ulong)*(undefined4 *)0x6612 >> 0x10);
            break;
          }
          puVar14 = puVar14 + 1;
          iVar8 = iVar8 + -1;
        } while (iVar8 != 0);
        do {
          iVar8 = local_12;
          puVar14 = local_c;
          if (*puVar13 == uVar1) {
            bVar6 = false;
            bVar4 = true;
            uVar15 = (undefined2)((ulong)*(undefined4 *)0x6612 >> 0x10);
            break;
          }
          iVar9 = iVar9 + -1;
          puVar13 = puVar13 + iVar12;
        } while (iVar9 != 0);
        do {
          iVar9 = local_10;
          puVar13 = local_e;
          if (*puVar14 == uVar1) {
            bVar6 = false;
            bVar5 = true;
            uVar15 = (undefined2)((ulong)*(undefined4 *)0x6612 >> 0x10);
            break;
          }
          iVar8 = iVar8 + -1;
          puVar14 = puVar14 + iVar12;
        } while (iVar8 != 0);
        do {
          if (*puVar13 == uVar1) {
            bVar6 = false;
            bVar3 = true;
            break;
          }
          iVar9 = iVar9 + -1;
          puVar13 = puVar13 + 1;
        } while (iVar9 != 0);
        if (bVar2) {
          local_8 = local_8 + -iVar12;
          local_c = local_c + -iVar12;
          local_12 = local_12 + 1;
        }
        if (bVar3) {
          local_e = local_e + iVar12;
          local_a = local_a + iVar12 * 2;
          local_12 = local_12 + 1;
        }
        if (bVar4) {
          local_8 = local_8 + -1;
          local_e = local_e + -1;
          local_10 = local_10 + 1;
        }
        if (bVar5) {
          local_c = local_c + 1;
          local_a = local_a + 2;
          local_10 = local_10 + 1;
        }
      } while (!bVar6);
      puVar14 = local_8 + iVar12 + 1;
      local_c = local_c + iVar12 + -1;
      iVar8 = local_a + iVar12 * -2 + -2;
      if (*(uint *)0x2080 < 0xfa) {
        *(int *)0x2080 = *(int *)0x2080 + 1;
        iVar9 = *(int *)0x2080 * 0xf;
        _local_20 = (int *)CONCAT22(unaff_DS,(undefined2 *)(iVar9 + 0x6611));
        *_local_20 = (int)puVar14;
        *(int *)(iVar9 + 0x6613) = iVar8;
        *(uint *)(iVar9 + 0x6615) = uVar1 | 0x8000;
        *(undefined1 *)(iVar9 + 0x6619) = 0;
        *(undefined1 *)(iVar9 + 0x661a) = 0;
        *(char *)(iVar9 + 0x6617) = param_3;
        *(char *)(iVar9 + 0x6618) = param_2;
        uVar10 = (int)param_2 >> 0xf;
        uVar11 = (int)param_3 >> 0xf;
        *(int *)(iVar9 + 0x661b) =
             (((int)param_3 ^ uVar11) - uVar11) + (((int)param_2 ^ uVar10) - uVar10);
        *(undefined1 *)(iVar9 + 0x661e) = 0;
        *(undefined1 *)(iVar9 + 0x661d) = 0;
        cVar7 = '\0';
        uVar15 = (undefined2)((ulong)*(undefined4 *)0x6612 >> 0x10);
        local_8 = puVar14;
        do {
          do {
            if (*puVar14 == uVar1) {
              cVar7 = cVar7 + '\x01';
              *puVar14 = *puVar14 ^ 0x8000;
            }
            puVar14 = puVar14 + 1;
          } while ((int)puVar14 <= (int)local_c);
          puVar14 = local_8 + iVar12;
          local_c = local_c + iVar12;
          local_8 = puVar14;
        } while ((int)local_c <= iVar8);
        *(char *)(*(int *)0x2080 * 0xf + 0x661f) = cVar7 << 1;
        *(undefined1 *)0x79c8 = 1;
      }
      else {
        *(undefined1 *)0x79c8 = 0;
      }
    }
    else {
      *param_1 = 1;
      if (*(uint *)0x207e < 0x640) {
        *(int *)0x207e = *(int *)0x207e + 1;
        *(uint *)((int)*(undefined4 *)0x6612 + param_4 * 2) =
             *(uint *)((int)*(undefined4 *)0x6612 + param_4 * 2) | 0x8000;
        iVar12 = *(int *)0x207e * 0xb;
        _local_20 = (int *)CONCAT22(unaff_DS,(int *)(iVar12 + 0x2093));
        *_local_20 = param_4;
        *(char *)(iVar12 + 0x2097) = param_3;
        *(char *)(iVar12 + 0x2098) = param_2;
        *(undefined1 *)(iVar12 + 0x209c) = *(undefined1 *)((int)*(undefined4 *)0xc1e0 + param_4);
        *(undefined1 *)(iVar12 + 0x2099) = 0;
        *(undefined1 *)(iVar12 + 0x209a) = 0;
        *(undefined1 *)(iVar12 + 0x209b) = 0;
        *(uint *)(iVar12 + 0x2095) = uVar1 | 0x8000;
        *(undefined1 *)(iVar12 + 0x209d) = 0;
        *(undefined1 *)0x79c8 = 1;
      }
      else {
        *(undefined1 *)0x79c8 = 0;
      }
    }
  }
  return;
}



// ================================================
// Function: FUN_1000_3a56 at 1000:3a56
// ================================================

void __cdecl16near FUN_1000_3a56(void)

{
  ulong uVar1;
  uint uVar2;
  undefined2 unaff_DS;
  bool bVar3;
  
  uVar1 = (ulong)(*(int *)0x2076 - 1) * 0xb;
  uVar2 = (uint)uVar1;
  do {
    if (*(int *)(uVar2 + 0x209e) == *(int *)0x2072) {
      *(int *)0x2074 = (int)((uVar1 & 0xffff0000 | (ulong)uVar2) / 0xb) + 1;
      return;
    }
    bVar3 = 10 < uVar2;
    uVar2 = uVar2 - 0xb;
  } while (bVar3);
  return;
}



// ================================================
// Function: FUN_1000_3a7e at 1000:3a7e
// ================================================

void __cdecl16near FUN_1000_3a7e(void)

{
  uint uVar1;
  uint uVar2;
  int iVar3;
  undefined2 unaff_DS;
  
  uVar1 = *(uint *)0x2074;
  if ((uVar1 & 0x8000) == 0) {
    *(undefined1 *)0x661e = 0;
  }
  else if ((uVar1 & 0x7fff) < 0x4000) {
    if (*(int *)0x2080 + -1 != -1) {
      uVar2 = (*(int *)0x2080 + -1) * 0xf + 4;
      iVar3 = *(int *)0x2080;
      do {
        if (*(uint *)(uVar2 + 0x6620) == uVar1) {
          *(undefined1 *)0x661e = *(undefined1 *)(uVar2 + 0x6622);
          *(int *)0x2074 = uVar2 / 0xf + 1;
          return;
        }
        uVar2 = uVar2 - 0xf;
        iVar3 = iVar3 + -1;
      } while (iVar3 != 0);
    }
  }
  else if (*(int *)0x207e + -1 != 0xc6) {
    uVar2 = (*(int *)0x207e + -1) * 0xb + 2;
    iVar3 = *(int *)0x207e + -199;
    do {
      if (*(uint *)(uVar2 + 0x209e) == uVar1) {
        *(undefined1 *)0x661e = *(undefined1 *)(uVar2 + 0x20a0);
        *(int *)0x2074 = uVar2 / 0xb + 1;
        return;
      }
      uVar2 = uVar2 - 0xb;
      iVar3 = iVar3 + -1;
    } while (iVar3 != 0);
  }
  return;
}



// ================================================
// Function: FUN_1000_3b18 at 1000:3b18
// ================================================

void __cdecl16near FUN_1000_3b18(void)

{
  uint uVar1;
  uint uVar2;
  int iVar3;
  undefined2 unaff_DS;
  
  uVar1 = *(uint *)0x2074;
  if ((uVar1 & 0x8000) == 0) {
    *(undefined1 *)0x661e = 0;
  }
  else if ((uVar1 & 0x7fff) < 0x4000) {
    if (*(int *)0x2080 + -1 != -1) {
      uVar2 = (*(int *)0x2080 + -1) * 0xf + 4;
      iVar3 = *(int *)0x2080;
      do {
        if (*(uint *)(uVar2 + 0x6620) == uVar1) {
          *(undefined1 *)0x661e = *(undefined1 *)(uVar2 + 0x6623);
          *(int *)0x2074 = uVar2 / 0xf + 1;
          return;
        }
        uVar2 = uVar2 - 0xf;
        iVar3 = iVar3 + -1;
      } while (iVar3 != 0);
    }
  }
  else if (*(int *)0x207e + -1 != 0xc6) {
    uVar2 = (*(int *)0x207e + -1) * 0xb + 2;
    iVar3 = *(int *)0x207e + -199;
    do {
      if (*(uint *)(uVar2 + 0x209e) == uVar1) {
        *(undefined1 *)0x661e = *(undefined1 *)(uVar2 + 0x20a1);
        *(int *)0x2074 = uVar2 / 0xb + 1;
        return;
      }
      uVar2 = uVar2 - 0xb;
      iVar3 = iVar3 + -1;
    } while (iVar3 != 0);
  }
  return;
}



// ================================================
// Function: FUN_1000_3bb2 at 1000:3bb2
// ================================================

void FUN_1000_3bb2(char *param_1,byte param_2)

{
  int iVar1;
  uint uVar2;
  char cVar3;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  bool bVar4;
  char local_e;
  byte local_d;
  int local_c;
  uint local_a;
  uint local_8;
  uint local_6;
  int local_4;
  
  local_4 = 0x1000;
  local_6 = 0x3bbd;
  FUN_1920_04df();
  local_a = (uint)param_2;
  local_6 = (uint)param_2 * (int)*param_1;
  local_4 = (int)local_6 >> 0xf;
  iVar1 = *(int *)0x2078;
  if (iVar1 != 0) {
    local_c = 1;
    while( true ) {
      local_8 = *(uint *)(local_c * 2 + 0x655c);
      if ((local_8 & 0x8000) == 0) {
        FUN_1000_370e(&local_e,unaff_SS,0,0,*(uint *)(local_c * 2 + 0x6598) >> 1);
        if (*(char *)0x79c8 == '\0') {
          return;
        }
        if (local_e == '\0') {
          *(undefined2 *)(local_c * 2 + 0x65d4) = *(undefined2 *)0x2080;
          local_a = *(byte *)(*(int *)0x2080 * 0xf + 0x661f) + local_a;
        }
        else {
          *(int *)(local_c * 2 + 0x65d4) = *(int *)0x207e + 20000;
          local_a = local_a + 1;
        }
      }
      else {
        *(uint *)0x2074 = local_8;
        FUN_1000_3a7e();
        if ((local_8 & 0x7fff) < 0x4000) {
          local_d = *(byte *)(*(int *)0x2074 * 0xf + 0x661f);
        }
        else {
          *(int *)0x2074 = *(int *)0x2074 + 20000;
          local_d = 1;
        }
        *(undefined2 *)(local_c * 2 + 0x65d4) = *(undefined2 *)0x2074;
        local_a = local_d + local_a;
        uVar2 = (uint)local_d * (int)*(char *)0x661e;
        bVar4 = CARRY2(uVar2,local_6);
        local_6 = uVar2 + local_6;
        local_4 = ((int)uVar2 >> 0xf) + local_4 + (uint)bVar4;
      }
      if (local_c == iVar1) break;
      local_c = local_c + 1;
    }
  }
  cVar3 = FUN_1920_0945();
  iVar1 = *(int *)0x2078;
  if (iVar1 != 0) {
    local_c = 1;
    while( true ) {
      uVar2 = *(uint *)(local_c * 2 + 0x65d4);
      if (uVar2 < 20000) {
        *(char *)(uVar2 * 0xf + 0x6617) = cVar3;
      }
      else {
        *(char *)((uVar2 + 0xb1e0) * 0xb + 0x2097) = cVar3;
      }
      if (local_c == iVar1) break;
      local_c = local_c + 1;
    }
  }
  *param_1 = cVar3;
  return;
}



// ================================================
// Function: FUN_1000_3d46 at 1000:3d46
// ================================================

void FUN_1000_3d46(char *param_1,byte param_2)

{
  int iVar1;
  uint uVar2;
  char cVar3;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  bool bVar4;
  char local_e;
  byte local_d;
  int local_c;
  uint local_a;
  uint local_8;
  uint local_6;
  int local_4;
  
  local_4 = 0x1000;
  local_6 = 0x3d51;
  FUN_1920_04df();
  local_a = (uint)param_2;
  local_6 = (uint)param_2 * (int)*param_1;
  local_4 = (int)local_6 >> 0xf;
  iVar1 = *(int *)0x2078;
  if (iVar1 != 0) {
    local_c = 1;
    while( true ) {
      local_8 = *(uint *)(local_c * 2 + 0x655c);
      if ((local_8 & 0x8000) == 0) {
        FUN_1000_370e(&local_e,unaff_SS,0,0,*(uint *)(local_c * 2 + 0x6598) >> 1);
        if (*(char *)0x79c8 == '\0') {
          return;
        }
        if (local_e == '\0') {
          *(undefined2 *)(local_c * 2 + 0x65d4) = *(undefined2 *)0x2080;
          local_a = *(byte *)(*(int *)0x2080 * 0xf + 0x661f) + local_a;
        }
        else {
          *(int *)(local_c * 2 + 0x65d4) = *(int *)0x207e + 20000;
          local_a = local_a + 1;
        }
      }
      else {
        *(uint *)0x2074 = local_8;
        FUN_1000_3b18();
        if ((local_8 & 0x7fff) < 0x4000) {
          local_d = *(byte *)(*(int *)0x2074 * 0xf + 0x661f);
        }
        else {
          *(int *)0x2074 = *(int *)0x2074 + 20000;
          local_d = 1;
        }
        *(undefined2 *)(local_c * 2 + 0x65d4) = *(undefined2 *)0x2074;
        local_a = local_d + local_a;
        uVar2 = (uint)local_d * (int)*(char *)0x661e;
        bVar4 = CARRY2(uVar2,local_6);
        local_6 = uVar2 + local_6;
        local_4 = ((int)uVar2 >> 0xf) + local_4 + (uint)bVar4;
      }
      if (local_c == iVar1) break;
      local_c = local_c + 1;
    }
  }
  cVar3 = FUN_1920_0945();
  iVar1 = *(int *)0x2078;
  if (iVar1 != 0) {
    local_c = 1;
    while( true ) {
      uVar2 = *(uint *)(local_c * 2 + 0x65d4);
      if (uVar2 < 20000) {
        *(char *)(uVar2 * 0xf + 0x6618) = cVar3;
      }
      else {
        *(char *)((uVar2 + 0xb1e0) * 0xb + 0x2098) = cVar3;
      }
      if (local_c == iVar1) break;
      local_c = local_c + 1;
    }
  }
  *param_1 = cVar3;
  return;
}



// ================================================
// Function: FUN_1000_3eda at 1000:3eda
// ================================================

void __cdecl16near FUN_1000_3eda(void)

{
  char cVar1;
  char cVar2;
  int iVar3;
  undefined2 unaff_DS;
  
  cVar1 = *(char *)0x78d2;
  iVar3 = 1;
  if (cVar1 < '\0') {
    iVar3 = -1;
  }
  cVar2 = *(char *)0x78d3 + cVar1;
  if (SCARRY1(*(char *)0x78d3,cVar1)) {
    cVar2 = cVar2 + -0x80;
    *(int *)0x2090 = *(int *)0x2090 + iVar3;
  }
  *(char *)0x78d3 = cVar2;
  cVar1 = *(char *)0x78d4;
  iVar3 = *(int *)0xc204;
  if (cVar1 < '\0') {
    iVar3 = -iVar3;
  }
  cVar2 = *(char *)0x78d5 + cVar1;
  if (SCARRY1(*(char *)0x78d5,cVar1)) {
    cVar2 = cVar2 + -0x80;
    *(int *)0x2090 = *(int *)0x2090 + iVar3;
  }
  *(char *)0x78d5 = cVar2;
  return;
}



// ================================================
// Function: FUN_1000_3f27 at 1000:3f27
// ================================================

void __cdecl16near FUN_1000_3f27(void)

{
  char cVar1;
  int iVar2;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  iVar2 = FUN_1920_13a8(*(undefined2 *)0xc204);
  iVar2 = iVar2 + *(int *)0xc204;
  if (*(char *)((int)*(undefined4 *)0xc1e0 + iVar2) == '\0') {
    cVar1 = FUN_1920_13a8(9);
    *(char *)0x79a3 = cVar1 + 'g';
    if (*(char *)0x79a3 == *(char *)0x79b4) {
      *(char *)0x79a3 = *(char *)0x79a3 + '\x01';
    }
    *(undefined1 *)((int)*(undefined4 *)0xc1e0 + iVar2) = *(undefined1 *)0x79a3;
    *(undefined2 *)((int)*(undefined4 *)0x6612 + iVar2 * 2) = *(undefined2 *)0x78c4;
    *(int *)0x78c4 = *(int *)0x78c4 + 1;
    FUN_1000_370e(0x79a3,unaff_DS,0,0,iVar2);
  }
  return;
}



// ================================================
// Function: FUN_1000_3fa6 at 1000:3fa6
// ================================================

void FUN_1000_3fa6(int param_1,undefined2 param_2,undefined1 param_3,undefined2 param_4,
                  undefined2 param_5)

{
  undefined2 uVar1;
  int iVar2;
  undefined2 uVar3;
  int iVar4;
  undefined2 *puVar5;
  undefined2 uVar6;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined1 uVar7;
  undefined1 uVar8;
  undefined2 in_stack_00000016;
  int in_stack_00000018;
  undefined2 in_stack_0000001a;
  undefined4 uVar9;
  undefined2 *puVar10;
  
  uVar9 = 0x10003fb1;
  FUN_1920_04df();
  FUN_1920_0f01(uVar9);
  FUN_1920_0efb();
  FUN_1920_0f01();
  FUN_1920_0efb();
  FUN_1920_0f01();
  FUN_1920_0efb();
  do {
    uVar8 = *(uint *)0x2076 == 0xc6;
    if (*(uint *)0x2076 < 0xc6) {
      *(int *)0x2076 = *(int *)0x2076 + 1;
      *(undefined1 *)(*(int *)0x2076 + 0x78d5) = param_3;
      puVar10 = (undefined2 *)CONCAT22(unaff_DS,(undefined2 *)(*(int *)0x2076 * 0xb + 0x2093));
      *puVar10 = in_stack_0000001a;
      uVar3 = 0;
      uVar1 = FUN_1920_0f0f();
      uVar6 = in_stack_00000016;
      FUN_1920_1021(0x1920,uVar1,param_5,uVar3);
      FUN_1920_0efb();
      uVar8 = FUN_1920_0f13();
      *(undefined1 *)((undefined2 *)puVar10 + 2) = uVar8;
      uVar3 = 0;
      uVar1 = FUN_1920_0f0f();
      iVar4 = in_stack_00000018;
      FUN_1920_1034(uVar1,uVar6,uVar3);
      FUN_1920_0efb();
      iVar2 = FUN_1920_0f13();
      iVar4 = iVar4 + (uint)(iVar2 != 0);
      uVar7 = iVar4 != 0;
      uVar8 = iVar4 == 0;
      uVar6 = (undefined2)((ulong)puVar10 >> 0x10);
      puVar5 = (undefined2 *)puVar10;
      *(char *)((int)puVar5 + 5) = -(char)iVar2;
      *(undefined1 *)(puVar5 + 3) = 0;
      *(undefined1 *)((int)puVar5 + 7) = 0;
      *(undefined1 *)((int)puVar5 + 9) = 0x75;
      *(undefined1 *)(puVar5 + 4) = *(undefined1 *)(param_1 + -3);
      *(undefined1 *)(puVar5 + 5) = *(undefined1 *)(param_1 + -4);
    }
    else {
      uVar7 = false;
    }
    FUN_1920_0ee9();
    param_5 = in_stack_00000016;
    FUN_1920_0f0b();
  } while ((bool)uVar7 || (bool)uVar8);
  return;
}



// ================================================
// Function: FUN_1000_414a at 1000:414a
// ================================================

void FUN_1000_414a(byte param_1,int param_2)

{
  undefined2 unaff_DS;
  undefined2 uVar1;
  
  uVar1 = 0x4155;
  FUN_1920_04df();
  if (1 < param_1) {
    *(undefined2 *)0x209a = 2;
    *(undefined2 *)0x209c = 1;
  }
  if (param_1 == 1) {
    uVar1 = 0x805;
    FUN_1000_3fa6(&stack0xfffe,0x50,1,0x86,0,0x3400,0x89,0,0x2a00,0,0,0,param_2);
    *(undefined2 *)0x2074 = 0xea74;
    *(undefined1 *)0x799f = 4;
  }
  else if (param_1 == 2) {
    uVar1 = 0x905;
    FUN_1000_3fa6(&stack0xfffe,0x6e,1,0x85,0,0x7000,0x89,0,0x3280,0,0,0,param_2);
    *(undefined2 *)0x2074 = 0xea7e;
    *(undefined1 *)0x799f = 5;
  }
  else if (param_1 == 3) {
    uVar1 = 0x903;
    FUN_1000_3fa6(&stack0xfffe,0x6e,9,0x85,0,0x3000,0x89,0,0x3280,0,0,0,param_2);
    *(undefined2 *)0x2074 = 0xea88;
    *(undefined1 *)0x799f = 6;
  }
  else if (param_1 == 4) {
    uVar1 = 0x3a0a;
    FUN_1000_0495(0x1aba,unaff_DS,0x1ab6,unaff_DS,0);
    FUN_1000_3fa6(&stack0xfffe,0x7e,0xffdd,0x84,0,0x7000,0x88,0,0x3400,0x87,0,0x3400,param_2);
    FUN_1000_3fa6(&stack0xfffe,0x7e,0xffdd,0x84,0,0x7000,0x87,0,0x3400,0,0,0,param_2 + 1);
    FUN_1000_3fa6(&stack0xfffe,0x7e,0xffdd,0x84,0,0x7000,0x89,0,0x700,0x88,0,0x3400,
                  param_2 + *(int *)0xc204);
    FUN_1000_3fa6(&stack0xfffe,0x7e,0xffdd,0x84,0,0x7000,0x89,0,0x3400,0x89,0,0xc00,
                  param_2 + *(int *)0xc204 + 1);
    *(undefined1 *)0x799f = 7;
    *(undefined2 *)0x2074 = 0xeace;
  }
  FUN_1000_165a(uVar1);
  return;
}



// ================================================
// Function: FUN_1000_432a at 1000:432a
// ================================================

void __cdecl16near FUN_1000_432a(void)

{
  byte bVar1;
  byte bVar2;
  undefined2 uVar3;
  int iVar4;
  int iVar5;
  undefined2 unaff_DS;
  undefined2 local_e;
  undefined2 local_a;
  undefined2 local_4;
  
  FUN_1920_04df();
  bVar1 = *(byte *)0x79f9;
  if (bVar1 != 0) {
    local_4 = 1;
    while( true ) {
      iVar5 = local_4 * 0x10;
      _local_a = (byte *)CONCAT22(unaff_DS,(byte *)(iVar5 + 0x79ea));
      bVar2 = *_local_a;
      _local_e = (int *)CONCAT22(unaff_DS,(int *)((uint)bVar2 * 8 + -0x3de2));
      if (*(char *)(iVar5 + 0x79ed) == -1) {
        *(byte *)(iVar5 + 0x79f0) = *(char *)(iVar5 + 0x79f0) + *(char *)(iVar5 + 0x79ec) & 0x7f;
        iVar4 = (*(byte *)(iVar5 + 0x79f0) + 0x20 & 0x7f) * 6;
        FUN_1920_0f0f(*(undefined2 *)(iVar4 + 0x7bda),*(undefined2 *)(iVar4 + 0x7bdc),
                      *(undefined2 *)(iVar4 + 0x7bde));
        FUN_1920_0efb();
        iVar4 = FUN_1920_0f13();
        *(int *)(iVar5 + 0x79f5) = *_local_e + *(int *)(iVar5 + 0x79f1) + iVar4;
        iVar4 = (uint)*(byte *)(iVar5 + 0x79f0) * 6;
        FUN_1920_0f0f(*(undefined2 *)(iVar4 + 0x7bda),*(undefined2 *)(iVar4 + 0x7bdc),
                      *(undefined2 *)(iVar4 + 0x7bde));
        FUN_1920_0efb();
        iVar4 = FUN_1920_0f13();
        *(int *)(iVar5 + 0x79f7) =
             *(int *)((uint)bVar2 * 8 + -0x3de0) + *(int *)(iVar5 + 0x79f3) + iVar4;
      }
      else {
        uVar3 = FUN_1920_092a();
        *(undefined2 *)(iVar5 + 0x79f5) = uVar3;
        iVar4 = FUN_1920_092a();
        *(int *)(iVar5 + 0x79f7) = *(char *)(iVar5 + 0x79f9) + iVar4;
      }
      if (local_4 == bVar1) break;
      local_4 = local_4 + 1;
    }
  }
  return;
}



// ================================================
// Function: FUN_1000_452a at 1000:452a
// ================================================

void FUN_1000_452a(undefined2 param_1,uint param_2)

{
  undefined1 *puVar1;
  undefined1 *puVar2;
  int iVar3;
  int iVar4;
  undefined1 *puVar5;
  undefined1 *puVar6;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  if (((int)param_2 >> 0xf < 0) || ((-1 < (int)param_2 && (param_2 < *(uint *)0x2076)))) {
    puVar6 = (undefined1 *)(*(int *)0x207c + (param_2 - 1) * 0xb);
    puVar5 = puVar6 + 0xb;
    for (iVar3 = (*(int *)0x2076 - param_2) * 0xb; iVar3 != 0; iVar3 = iVar3 + -1) {
      puVar2 = puVar6;
      puVar6 = puVar6 + 1;
      puVar1 = puVar5;
      puVar5 = puVar5 + 1;
      *puVar2 = *puVar1;
    }
    iVar3 = *(int *)0x2076 - param_2;
    iVar4 = param_2 - 1;
    do {
      *(undefined1 *)(iVar4 + 0x78d6) = *(undefined1 *)(iVar4 + 0x78d7);
      iVar4 = iVar4 + 1;
      iVar3 = iVar3 + -1;
    } while (iVar3 != 0);
  }
  *(int *)0x2076 = *(int *)0x2076 + -1;
  return;
}



// ================================================
// Function: FUN_1000_458d at 1000:458d
// ================================================

void FUN_1000_458d(int param_1,uint param_2)

{
  undefined1 *puVar1;
  undefined1 *puVar2;
  int iVar3;
  int iVar4;
  undefined1 *puVar5;
  undefined1 *puVar6;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  iVar4 = (int)param_2 >> 0xf;
  if ((iVar4 < 0) || ((-1 < (int)param_2 && (param_2 < *(uint *)0x207e)))) {
    puVar6 = (undefined1 *)(*(int *)0x207c + (param_2 - 1) * 0xb);
    puVar5 = puVar6 + 0xb;
    for (iVar3 = (*(int *)0x207e - param_2) * 0xb; iVar3 != 0; iVar3 = iVar3 + -1) {
      puVar2 = puVar6;
      puVar6 = puVar6 + 1;
      puVar1 = puVar5;
      puVar5 = puVar5 + 1;
      *puVar2 = *puVar1;
    }
  }
  if (((int)param_2 < 0 && iVar4 < 1) || ((iVar4 < 1 && (param_2 <= *(uint *)(param_1 + -2))))) {
    *(int *)(param_1 + -2) = *(int *)(param_1 + -2) + -1;
  }
  *(int *)0x207e = *(int *)0x207e + -1;
  return;
}



// ================================================
// Function: FUN_1000_45fa at 1000:45fa
// ================================================

void __cdecl16near FUN_1000_45fa(void)

{
  char *pcVar1;
  byte bVar2;
  char cVar3;
  uint uVar4;
  int iVar5;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  bool bVar6;
  int *local_1c;
  byte local_16;
  char local_14;
  byte local_13;
  char local_12;
  char local_11;
  uint local_e;
  byte *local_c;
  char *local_a;
  uint local_8;
  uint local_6;
  uint local_4;
  
  local_4 = 0x1000;
  local_6 = 0x4605;
  FUN_1920_04df();
  local_4 = *(int *)0x2076;
  do {
    if (local_4 == 0) {
      if (199 < *(uint *)0x207e) {
        for (local_4 = 200; local_4 <= *(uint *)0x207e; local_4 = local_4 + 1) {
          iVar5 = local_4 * 0xb;
          _local_1c = (int *)CONCAT22(unaff_DS,(undefined2 *)(iVar5 + 0x2093));
          local_a = (char *)*_local_1c;
          *(undefined1 *)0x78d2 = *(undefined1 *)(iVar5 + 0x2097);
          *(undefined1 *)0x78d4 = *(undefined1 *)(iVar5 + 0x2098);
          *(undefined1 *)0x78d3 = *(undefined1 *)(iVar5 + 0x2099);
          *(undefined1 *)0x78d5 = *(undefined1 *)(iVar5 + 0x209a);
          local_16 = *(byte *)(iVar5 + 0x209c);
          local_e = *(uint *)(local_4 * 0xb + 0x2095);
          if ((0xffbc < local_e) && (local_16 < 0x67)) {
            local_16 = 0x76;
            *(undefined1 *)(local_4 * 0xb + 0x209c) = 0x76;
            *(undefined2 *)0x2074 = 0x27;
            *(undefined1 *)0x799f = 5;
            FUN_1000_165a();
          }
          if (0x75 < local_16) {
            local_16 = local_16 + 1;
            pcVar1 = (char *)(local_4 * 0xb + 0x209c);
            *pcVar1 = *pcVar1 + '\x01';
            if (local_16 == 0x79) {
              if (local_e < 0xffbd) {
                local_13 = 0xff;
              }
              else {
                cVar3 = FUN_1920_13a8(5);
                local_13 = cVar3 + 0x6b;
              }
              *(byte *)(local_4 * 0xb + 0x209c) = local_13;
              local_16 = local_13;
            }
            local_a[(int)*(undefined4 *)0xc1e0] = local_16;
          }
          if (*local_a == -1) {
            *local_a = '\0';
            FUN_1000_458d(&stack0xfffe,local_4);
            *(undefined2 *)((int)*(undefined4 *)0x6612 + (int)local_a * 2) = 0;
            if (*(int *)((int)*(undefined4 *)0x6612 + ((int)local_a - *(int *)0xc204) * 2) != 0) {
              FUN_1000_370e(&local_14,unaff_SS,0,0,(int)local_a - *(int *)0xc204);
            }
          }
          else {
            *(undefined2 *)0x2090 = 0;
            FUN_1000_3eda();
            bVar6 = *(int *)0x2090 == 0;
            if (local_a[*(int *)0xc204] == '\0') {
              if (*(char *)0x78d4 < '{') {
                *(char *)0x78d4 = *(char *)0x78d4 + '\x04';
              }
              *(undefined1 *)(local_4 * 0xb + 0x209b) = 0;
            }
            else {
              if (!bVar6) {
                uVar4 = (int)*(char *)0x78d2 >> 0xf;
                if ((int)(((int)*(char *)0x78d2 ^ uVar4) - uVar4) < 1) {
                  *(undefined1 *)0x78d2 = 0;
                }
                else if (*(char *)0x78d2 < '\x01') {
                  *(char *)0x78d2 = *(char *)0x78d2 + '\x01';
                }
                else {
                  *(char *)0x78d2 = *(char *)0x78d2 + -1;
                }
              }
              if (((('\0' < *(char *)0x78d4) && ('<' < *(char *)0x78d4)) && (0x66 < local_16)) &&
                 (2 < (*(int *)0x78c2 + local_4) % 6)) {
                local_a[(int)*(undefined4 *)0xc1e0] = 'v';
                *(undefined1 *)(local_4 * 0xb + 0x209c) = 0x76;
                local_16 = 0x76;
                *(undefined1 *)0x799f = 2;
                *(undefined2 *)0x2074 = 0x21;
                FUN_1000_165a();
              }
            }
            if (*(int *)0x2090 == 0) {
              bVar6 = true;
            }
            else {
              local_c = (byte *)(local_a + *(int *)0x2090);
              local_13 = *local_c;
              if (local_13 == 0) {
                *(undefined1 *)(local_4 * 0xb + 0x209b) = 0;
                *local_c = local_16;
                *local_a = '\0';
                *(uint *)((int)*(undefined4 *)0x6612 + (int)local_c * 2) = local_e;
                *(undefined2 *)((int)*(undefined4 *)0x6612 + (int)local_a * 2) = 0;
                *(undefined2 *)(local_4 * 0xb + 0x2093) = local_c;
                if (((*(int *)0x2090 >> 0xf != -(uint)(*(int *)0xc204 != 0)) ||
                    (*(int *)0x2090 != -*(int *)0xc204)) &&
                   ((local_8 = *(uint *)((int)*(undefined4 *)0x6612 +
                                        ((int)local_a - *(int *)0xc204) * 2), local_8 != 0 &&
                    (local_8 < 0x8000)))) {
                  FUN_1000_370e(&local_13,unaff_SS,0,0,(int)local_a - *(int *)0xc204);
                  *(byte *)(local_4 * 0xb + 0x209d) = *(byte *)(local_4 * 0xb + 0x209d) | 0x80;
                }
              }
              else {
                bVar6 = true;
                if ('\0' < *(char *)0x78d4) {
                  cVar3 = FUN_1920_13a8(0x1e);
                  *(char *)0x78d2 = *(char *)0x78d2 + cVar3 + -0xf;
                  iVar5 = FUN_1920_13a8(8);
                  *(int *)0x2074 = iVar5 + -0x159f;
                  *(undefined1 *)0x799f = 1;
                  FUN_1000_165a();
                  *(undefined1 *)0x78d4 = 0;
                }
                local_6 = *(uint *)((int)*(undefined4 *)0x6612 + (int)local_c * 2);
                if (local_6 == 0) {
                  *(undefined1 *)0x78d2 = 0;
                }
                else {
                  *(undefined2 *)0x2078 = 1;
                  *(int *)0x659a = (int)local_c << 1;
                  *(uint *)0x655e = local_6;
                  FUN_1000_3bb2(0x78d2,unaff_DS,1);
                  *(uint *)0x655e = local_6 | 0x8000;
                  FUN_1000_3d46(0x78d4,unaff_DS,1);
                }
              }
            }
            iVar5 = local_4 * 0xb;
            _local_1c = (int *)CONCAT22(unaff_DS,(int *)(iVar5 + 0x2093));
            *(undefined1 *)(iVar5 + 0x2098) = *(undefined1 *)0x78d4;
            *(undefined1 *)(iVar5 + 0x2099) = *(undefined1 *)0x78d3;
            *(undefined1 *)(iVar5 + 0x209a) = *(undefined1 *)0x78d5;
            *(undefined1 *)(iVar5 + 0x2097) = *(undefined1 *)0x78d2;
            if (bVar6) {
              *(char *)(iVar5 + 0x209b) = *(char *)(iVar5 + 0x209b) + '\x01';
            }
            if (*(char *)(iVar5 + 0x209b) == 'd') {
              *(uint *)((int)*(undefined4 *)0x6612 + *_local_1c * 2) =
                   *(uint *)((int)*(undefined4 *)0x6612 + *_local_1c * 2) & 0x7fff;
              FUN_1000_458d(&stack0xfffe,local_4);
            }
          }
        }
      }
      return;
    }
    iVar5 = local_4 * 0xb;
    _local_1c = (int *)CONCAT22(unaff_DS,(undefined2 *)(iVar5 + 0x2093));
    local_a = (char *)*_local_1c;
    *(undefined1 *)0x78d2 = *(undefined1 *)(iVar5 + 0x2097);
    *(undefined1 *)0x78d4 = *(undefined1 *)(iVar5 + 0x2098);
    *(undefined1 *)0x78d3 = *(undefined1 *)(iVar5 + 0x2099);
    *(undefined1 *)0x78d5 = *(undefined1 *)(iVar5 + 0x209a);
    *(undefined2 *)0x2090 = 0;
    FUN_1000_3eda();
    if (*(int *)0x2090 != 0) {
      local_c = (byte *)(local_a + *(int *)0x2090);
      local_13 = *local_c;
      if (local_13 == 0x66) {
        if (*(uint *)((int)*(undefined4 *)0x6612 + (int)local_c * 2) < 0x8000) {
          local_c[(int)*(undefined4 *)0xc1e0] = 0;
          *(undefined2 *)((int)*(undefined4 *)0x6612 + (int)local_c * 2) = 0;
        }
        else {
          local_c[(int)*(undefined4 *)0xc1e0] = 0xff;
        }
        FUN_1000_414a(1,local_c);
      }
      if ((local_13 == 0) || (local_13 == 0x75)) {
        if (*local_a == 'u') {
          *local_a = '\0';
        }
        *local_c = *(byte *)(local_4 * 0xb + 0x209c);
        *(undefined2 *)(local_4 * 0xb + 0x2093) = local_c;
      }
      else {
        local_6 = *(uint *)((int)*(undefined4 *)0x6612 + (int)local_c * 2);
        if (local_6 != 0) {
          if ((local_6 & 0x8000) == 0) {
            FUN_1000_370e(&local_14,unaff_SS,0,0,local_c);
            local_11 = '\0';
            local_12 = '\0';
            if (*(char *)0x79c8 == '\0') goto LAB_1000_48aa;
            if (local_14 == '\0') {
              *(undefined2 *)0x2074 = *(undefined2 *)0x2080;
              local_13 = *(byte *)(*(int *)0x2074 * 0xf + 0x661f);
            }
            else {
              local_13 = 1;
              *(undefined2 *)0x2074 = *(undefined2 *)0x207e;
            }
          }
          else {
            *(uint *)0x2074 = local_6;
            FUN_1000_3a7e();
            local_11 = *(char *)0x661e;
            *(uint *)0x2074 = local_6;
            FUN_1000_3b18();
            local_12 = *(char *)0x661e;
            if ((local_6 & 0x7fff) < 0x4000) {
              local_13 = *(byte *)(*(int *)0x2074 * 0xf + 0x661f);
            }
            else {
              local_13 = 1;
              if (*(char *)(local_4 * 0xb + 0x209d) != '\0') {
                local_c[(int)*(undefined4 *)0xc1e0] = 0xff;
              }
            }
          }
          bVar2 = *(byte *)(local_4 + 0x78d5);
          local_11 = (char)((int)((uint)bVar2 * (int)*(char *)0x78d2 +
                                 (uint)local_13 * (int)local_11) /
                           (int)((uint)local_13 + (uint)bVar2));
          local_12 = (char)((int)((uint)bVar2 * (int)*(char *)0x78d4 +
                                 (uint)local_13 * (int)local_12) /
                           (int)((uint)local_13 + (uint)bVar2));
          if ((local_6 & 0x7fff) < 0x4000) {
            iVar5 = *(int *)0x2074;
            *(char *)(iVar5 * 0xf + 0x6617) = local_11;
            *(char *)(iVar5 * 0xf + 0x6618) = local_12;
          }
          else {
            iVar5 = *(int *)0x2074;
            *(char *)(iVar5 * 0xb + 0x2097) = local_11;
            *(char *)(iVar5 * 0xb + 0x2098) = local_12;
          }
        }
      }
    }
LAB_1000_48aa:
    iVar5 = local_4 * 0xb;
    _local_1c = (int *)CONCAT22(unaff_DS,(undefined2 *)(iVar5 + 0x2093));
    *(undefined1 *)(iVar5 + 0x2098) = *(undefined1 *)0x78d4;
    *(undefined1 *)(iVar5 + 0x2099) = *(undefined1 *)0x78d3;
    *(undefined1 *)(iVar5 + 0x209a) = *(undefined1 *)0x78d5;
    *(undefined1 *)(iVar5 + 0x2097) = *(undefined1 *)0x78d2;
    if (*(char *)(iVar5 + 0x209d) != '\0') {
      *(char *)(iVar5 + 0x209d) = *(char *)(iVar5 + 0x209d) + -1;
    }
    *(char *)(iVar5 + 0x209b) = *(char *)(iVar5 + 0x209b) + -1;
    if (*(char *)(iVar5 + 0x209b) == '\0') {
      if (*(char *)*_local_1c == 'u') {
        *(undefined1 *)*_local_1c = 0;
      }
      FUN_1000_452a(&stack0xfffe,local_4);
    }
    local_4 = local_4 + -1;
  } while( true );
}



// ================================================
// Function: FUN_1000_4d3c at 1000:4d3c
// ================================================

uint __cdecl16near FUN_1000_4d3c(void)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  uint *puVar6;
  uint uVar7;
  uint *puVar8;
  undefined2 uVar9;
  undefined2 unaff_DS;
  
  *(undefined2 *)0x2074 = 10000;
  *(undefined2 *)0x2072 = 0;
  *(undefined1 *)0x661e = 0;
  uVar9 = (undefined2)((ulong)*(undefined4 *)0x6612 >> 0x10);
  iVar2 = *(int *)0x2090;
  *(int *)0x2092 = iVar2;
  iVar2 = iVar2 << 1;
  *(int *)0x2090 = iVar2;
  puVar6 = (uint *)(*(int *)0x2068 << 1);
  iVar5 = *(int *)0x205c;
  do {
    iVar4 = *(int *)0x205a;
    puVar8 = puVar6;
    do {
      uVar3 = *puVar8;
      if (uVar3 == *(uint *)0x205e) {
        piVar1 = (int *)(iVar2 + (int)puVar8);
        uVar7 = (uint)puVar8 >> 1;
        uVar3 = CONCAT11((char)(uVar3 >> 8),*(char *)(*(int *)0x2092 + uVar7));
        puVar8 = (uint *)(uVar7 << 1);
        iVar2 = *(int *)0x2090;
        uVar9 = *(undefined2 *)0x206e;
        if ((*(char *)(*(int *)0x2092 + uVar7) != '\0') && (*piVar1 != *(int *)0x205e)) {
          *(undefined1 *)0x661e = 1;
          uVar3 = (uVar7 + *(int *)0x2092) / *(uint *)0xc204;
          uVar7 = (uVar7 + *(int *)0x2092) % *(uint *)0xc204;
          if ((int)uVar7 < *(int *)0x2074) {
            *(uint *)0x2074 = uVar7;
          }
          if (*(int *)0x2072 < (int)uVar7) {
            *(uint *)0x2072 = uVar7;
          }
        }
      }
      puVar8 = puVar8 + 1;
      iVar4 = iVar4 + -1;
    } while (iVar4 != 0);
    puVar6 = (uint *)((int)puVar6 + *(int *)0x2066);
    iVar5 = iVar5 + -1;
  } while (iVar5 != 0);
  return uVar3;
}



// ================================================
// Function: FUN_1000_4dd3 at 1000:4dd3
// ================================================

void __cdecl16near FUN_1000_4dd3(void)

{
  int *piVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  uint uVar6;
  int *piVar7;
  undefined2 uVar8;
  undefined2 unaff_DS;
  
  *(undefined2 *)0x2074 = 10000;
  *(undefined2 *)0x2072 = 0;
  *(undefined1 *)0x661e = 0;
  uVar8 = (undefined2)((ulong)*(undefined4 *)0x6612 >> 0x10);
  iVar2 = *(int *)0x2090;
  *(int *)0x2092 = iVar2;
  iVar2 = iVar2 << 1;
  *(int *)0x2090 = iVar2;
  piVar5 = (int *)(*(int *)0x2068 << 1);
  iVar4 = *(int *)0x205c;
  do {
    iVar3 = *(int *)0x205a;
    piVar7 = piVar5;
    do {
      if (*piVar7 == *(int *)0x205e) {
        piVar1 = (int *)(iVar2 + (int)piVar7);
        uVar6 = (uint)piVar7 >> 1;
        piVar7 = (int *)(uVar6 << 1);
        iVar2 = *(int *)0x2090;
        uVar8 = *(undefined2 *)0x206e;
        if ((*(char *)(*(int *)0x2092 + uVar6) != '\0') && (*piVar1 != *(int *)0x205e)) {
          *(undefined1 *)0x661e = 1;
          return;
        }
      }
      piVar7 = piVar7 + 1;
      iVar3 = iVar3 + -1;
    } while (iVar3 != 0);
    piVar5 = (int *)((int)piVar5 + *(int *)0x2066);
    iVar4 = iVar4 + -1;
    if (iVar4 == 0) {
      return;
    }
  } while( true );
}



// ================================================
// Function: FUN_1000_4e48 at 1000:4e48
// ================================================

int __cdecl16near FUN_1000_4e48(void)

{
  int *piVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int *piVar10;
  int *piVar11;
  undefined2 uVar12;
  undefined2 unaff_DS;
  
  *(undefined1 *)0x661e = 0;
  uVar12 = (undefined2)((ulong)*(undefined4 *)0x6612 >> 0x10);
  iVar5 = *(int *)0x2090;
  *(int *)0x2092 = iVar5;
  iVar5 = iVar5 << 1;
  *(int *)0x2090 = iVar5;
  piVar10 = (int *)(*(int *)0x2068 << 1);
  iVar8 = *(int *)0x205c;
  do {
    iVar7 = *(int *)0x205a;
    piVar11 = piVar10;
    do {
      iVar6 = *piVar11;
      if (iVar6 == *(int *)0x205e) {
        iVar3 = *(int *)(iVar5 + (int)piVar11);
        cVar2 = *(char *)(*(int *)0x2092 + ((uint)piVar11 >> 1));
        iVar6 = CONCAT11((char)((uint)iVar6 >> 8),cVar2);
        piVar11 = (int *)(((uint)piVar11 >> 1) * 2);
        iVar5 = *(int *)0x2090;
        uVar12 = *(undefined2 *)0x206e;
        if (((cVar2 != '\0') && (iVar3 != *(int *)0x205e)) &&
           (*(undefined1 *)0x661e = 1, iVar3 != 0)) {
          iVar6 = (int)piVar11 + iVar5;
          iVar9 = *(int *)0x2078 * 2;
          iVar4 = iVar9;
          do {
            if (iVar4 == 0) {
              *(int *)(iVar9 + 0x655e) = iVar3;
              *(int *)(iVar9 + 0x659a) = iVar6;
              *(int *)0x2078 = *(int *)0x2078 + 1;
              break;
            }
            piVar1 = (int *)(iVar4 + 0x655c);
            iVar4 = iVar4 + -2;
          } while (*piVar1 != iVar3);
        }
      }
      piVar11 = piVar11 + 1;
      iVar7 = iVar7 + -1;
    } while (iVar7 != 0);
    piVar10 = (int *)((int)piVar10 + *(int *)0x2066);
    iVar8 = iVar8 + -1;
    if (iVar8 == 0) {
      return iVar6;
    }
  } while( true );
}



// ================================================
// Function: FUN_1000_4ee7 at 1000:4ee7
// ================================================

void __cdecl16near FUN_1000_4ee7(void)

{
  undefined1 uVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  int in_BX;
  int *piVar5;
  undefined1 *puVar6;
  int *piVar7;
  undefined2 uVar8;
  undefined2 unaff_DS;
  
  *(undefined1 *)0x208c = 0;
  *(undefined2 *)0x2078 = 0;
  FUN_1000_4e48();
  if (*(char *)0x661e != '\x01') {
    uVar8 = (undefined2)((ulong)*(undefined4 *)0x6612 >> 0x10);
    *(undefined1 *)0x208c = 1;
    iVar4 = *(int *)0x2092;
    if (-*(int *)0xc204 == iVar4) {
      *(undefined2 *)0x2060 = *(undefined2 *)0x2068;
      *(undefined2 *)0x2062 = 2;
      *(undefined2 *)0x2064 = *(undefined2 *)0x2066;
    }
    else if (iVar4 == *(int *)0xc204) {
      *(undefined2 *)0x2060 = *(undefined2 *)0x206c;
      *(undefined2 *)0x2062 = 0xfffe;
      *(int *)0x2064 = -*(int *)0x2066;
    }
    else if (iVar4 == 1) {
      *(undefined2 *)0x2060 = *(undefined2 *)0x206a;
      *(undefined2 *)0x2062 = 0xfffe;
      *(undefined2 *)0x2064 = *(undefined2 *)0x2066;
    }
    else {
      *(undefined2 *)0x2060 = *(undefined2 *)0x2068;
      *(undefined2 *)0x2062 = 2;
      *(undefined2 *)0x2064 = *(undefined2 *)0x2066;
    }
    piVar5 = (int *)(*(int *)0x2060 << 1);
    iVar4 = *(int *)0x205c;
    do {
      iVar3 = *(int *)0x205a;
      piVar7 = piVar5;
      do {
        if (*piVar7 == *(int *)0x205e) {
          *(int *)(in_BX + (int)piVar7) = *piVar7;
          *piVar7 = 0;
          puVar6 = (undefined1 *)((uint)piVar7 >> 1);
          iVar2 = *(int *)0x2092;
          uVar8 = *(undefined2 *)0xc1fe;
          LOCK();
          uVar1 = *puVar6;
          *puVar6 = 0;
          UNLOCK();
          puVar6[iVar2] = uVar1;
          in_BX = *(int *)0x2090;
          piVar7 = (int *)((int)puVar6 << 1);
          uVar8 = *(undefined2 *)0x206e;
        }
        piVar7 = (int *)((int)piVar7 + *(int *)0x2062);
        iVar3 = iVar3 + -1;
      } while (iVar3 != 0);
      piVar5 = (int *)((int)piVar5 + *(int *)0x2064);
      iVar4 = iVar4 + -1;
    } while (iVar4 != 0);
    *(int *)0x2068 = *(int *)0x2068 + *(int *)0x2092;
    *(int *)0x206a = *(int *)0x206a + *(int *)0x2092;
    *(int *)0x206c = *(int *)0x206c + *(int *)0x2092;
  }
  return;
}



// ================================================
// Function: FUN_1000_4fda at 1000:4fda
// ================================================

void __cdecl16near FUN_1000_4fda(void)

{
  int iVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  char *pcVar5;
  int *piVar6;
  int *piVar7;
  undefined2 uVar8;
  undefined2 unaff_DS;
  undefined2 uVar9;
  
  FUN_1920_04df();
  uVar8 = (undefined2)((ulong)*(undefined4 *)0x6612 >> 0x10);
  piVar7 = (int *)*(undefined2 *)0x2068;
  iVar4 = *(int *)0x205c;
  do {
    iVar3 = *(int *)0x205a;
    piVar6 = piVar7;
    do {
      if (*piVar6 == *(int *)0x205e) {
        *piVar6 = *(int *)0x78c4;
        *(int *)0x78c8 = *(int *)0x78c8 + 1;
        pcVar5 = (char *)((uint)piVar6 >> 1);
        *pcVar5 = ((byte)*(undefined2 *)0x78c2 & 2) + 0x47;
        *(undefined2 *)0x2074 = pcVar5;
        piVar6 = (int *)((int)pcVar5 << 1);
        *(int *)0x78c4 = *(int *)0x78c4 + 1;
        uVar9 = *(undefined2 *)0x2074;
        iVar1 = FUN_1920_13a8(0x14);
        iVar1 = *(char *)0x78d2 + iVar1 + -10;
        iVar2 = FUN_1920_13a8(0x28);
        FUN_1000_370e(0x79a3,unaff_DS,*(char *)0x78d4 - iVar2,iVar1,uVar9);
      }
      piVar6 = piVar6 + 1;
      iVar3 = iVar3 + -1;
    } while (iVar3 != 0);
    piVar7 = (int *)((int)piVar7 + *(int *)0x2066);
    iVar4 = iVar4 + -1;
  } while (iVar4 != 0);
  return;
}



// ================================================
// Function: FUN_1000_508b at 1000:508b
// ================================================

void FUN_1000_508b(undefined2 param_1,uint param_2)

{
  undefined1 *puVar1;
  undefined1 *puVar2;
  int iVar3;
  int iVar4;
  undefined1 *puVar5;
  undefined1 *puVar6;
  uint *puVar7;
  undefined2 uVar8;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  if (param_2 < *(uint *)0x2080) {
    puVar6 = (undefined1 *)(*(int *)0x207a + (param_2 - 1) * 0xf);
    puVar5 = puVar6 + 0xf;
    for (iVar4 = (*(int *)0x2080 - param_2) * 0xf; iVar4 != 0; iVar4 = iVar4 + -1) {
      puVar2 = puVar6;
      puVar6 = puVar6 + 1;
      puVar1 = puVar5;
      puVar5 = puVar5 + 1;
      *puVar2 = *puVar1;
    }
  }
  uVar8 = (undefined2)((ulong)*(undefined4 *)0x6612 >> 0x10);
  do {
    puVar7 = (uint *)*(undefined2 *)0x2068;
    do {
      if (*puVar7 == *(uint *)0x205e) {
        *puVar7 = *puVar7 & 0x7fff;
      }
      puVar7 = puVar7 + 1;
    } while ((int)puVar7 <= *(int *)0x206a);
    *(int *)0x2068 = *(int *)0x2068 + *(int *)0x2066;
    iVar4 = *(int *)0x206a;
    iVar3 = *(int *)0x2066;
    *(int *)0x206a = iVar4 + iVar3;
  } while (iVar4 + iVar3 <= *(int *)0x206c);
  *(int *)0x2080 = *(int *)0x2080 + -1;
  return;
}



// ================================================
// Function: FUN_1000_5102 at 1000:5102
// ================================================

void __cdecl16near FUN_1000_5102(void)

{
  char *pcVar1;
  char cVar2;
  undefined1 extraout_AH;
  undefined1 extraout_AH_00;
  undefined1 extraout_AH_01;
  undefined1 uVar3;
  uint uVar4;
  uint uVar5;
  undefined2 uVar6;
  int iVar7;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  bool bVar8;
  uint *local_18;
  byte local_11;
  char local_10;
  char local_f;
  char local_e;
  char local_d;
  uint local_c;
  uint local_a;
  uint local_8;
  int local_6;
  int local_4;
  
  local_4 = 0x1000;
  local_6 = 0x510d;
  FUN_1920_04df();
  local_4 = *(int *)0x2080;
  *(int *)0x2066 = *(int *)0xc204 << 1;
  do {
    iVar7 = local_4 * 0xf;
    _local_18 = (uint *)CONCAT22(unaff_DS,(uint *)(iVar7 + 0x6611));
    *(uint *)0x2068 = *_local_18 >> 1;
    local_d = *(char *)(iVar7 + 0x6617);
    local_e = *(char *)(iVar7 + 0x6618);
    cVar2 = *(char *)(iVar7 + 0x6619);
    local_10 = *(char *)(iVar7 + 0x661a);
    *(uint *)0x206c = *(uint *)(iVar7 + 0x6613) >> 1;
    *(undefined2 *)0x205e = *(undefined2 *)(iVar7 + 0x6615);
    *(char *)0x78d2 = local_d;
    *(char *)0x78d4 = local_e;
    *(int *)0x206a =
         (*(uint *)0x206c % *(uint *)0xc204 + *(int *)0x2068) - *(uint *)0x2068 % *(uint *)0xc204;
    *(int *)0x205a = (*(int *)0x206a - *(int *)0x2068) + 1;
    *(int *)0x205c = (uint)(*(int *)0x206c - *(int *)0x206a) / *(uint *)0xc204 + 1;
    *(undefined2 *)0x2090 = 0;
    *(undefined1 *)0x208c = 0;
    uVar6 = 1;
    if (local_d < '\0') {
      uVar6 = 0xffff;
    }
    local_f = cVar2 + local_d;
    if (SCARRY1(cVar2,local_d)) {
      local_f = local_f + -0x80;
      *(undefined2 *)0x2090 = uVar6;
    }
    if (*(int *)0x2090 != 0) {
      if (*(byte *)(local_4 * 0xf + 0x661d) < 0x80) {
        uVar6 = *(undefined2 *)0x2090;
        *(int *)0x2090 = -*(int *)0xc204;
        *(undefined2 *)0x2078 = 0;
        FUN_1000_4e48();
        if ((*(char *)0x661e == '\x01') && (iVar7 = *(int *)0x2078, iVar7 != 0)) {
          local_6 = 1;
          while( true ) {
            if (*(uint *)(local_6 * 2 + 0x655c) < 0x8000) {
              FUN_1000_370e(&local_11,unaff_SS,1,0,*(uint *)(local_6 * 2 + 0x6598) >> 1);
            }
            if (local_6 == iVar7) break;
            local_6 = local_6 + 1;
          }
        }
        *(undefined2 *)0x2090 = uVar6;
      }
      else {
        *(byte *)(local_4 * 0xf + 0x661d) = *(byte *)(local_4 * 0xf + 0x661d) & 0x7f;
      }
      FUN_1000_4ee7();
      if (*(int *)0x2078 != 0) {
        FUN_1000_3bb2(&local_d,unaff_SS,
                      CONCAT11(extraout_AH,*(undefined1 *)(local_4 * 0xf + 0x661f)));
      }
    }
    *(undefined2 *)0x2090 = 0;
    iVar7 = *(int *)0xc204;
    if (local_e < '\0') {
      iVar7 = -iVar7;
    }
    bVar8 = SCARRY1(local_10,local_e);
    local_10 = local_10 + local_e;
    if (bVar8) {
      local_10 = local_10 + -0x80;
      *(int *)0x2090 = iVar7;
    }
    if (*(int *)0x2090 != 0) {
      if ((*(int *)0x2090 < 1) || (0x7f < *(byte *)(local_4 * 0xf + 0x661d))) {
        if (*(int *)0x2090 < 1) {
          *(byte *)(local_4 * 0xf + 0x661d) = *(byte *)(local_4 * 0xf + 0x661d) & 0x7f;
        }
      }
      else {
        *(byte *)(local_4 * 0xf + 0x661d) = *(byte *)(local_4 * 0xf + 0x661d) | 0x80;
        uVar6 = *(undefined2 *)0x2090;
        *(int *)0x2090 = -*(int *)0xc204;
        *(undefined2 *)0x2078 = 0;
        FUN_1000_4e48();
        if ((*(char *)0x661e == '\x01') && (iVar7 = *(int *)0x2078, iVar7 != 0)) {
          local_6 = 1;
          while( true ) {
            if (*(uint *)(local_6 * 2 + 0x655c) < 0x8000) {
              FUN_1000_370e(&local_11,unaff_SS,1,0,*(uint *)(local_6 * 2 + 0x6598) >> 1);
            }
            if (local_6 == iVar7) break;
            local_6 = local_6 + 1;
          }
        }
        *(undefined2 *)0x2090 = uVar6;
      }
      FUN_1000_4ee7();
      uVar3 = extraout_AH_00;
      if ((*(char *)0x661e == '\x01') && ('\0' < local_e)) {
        local_e = '\0';
        *(undefined2 *)0x209a = 3;
        *(undefined2 *)0x209c = 1;
        iVar7 = FUN_1920_13a8(8);
        *(int *)0x2074 = iVar7 + -0x159f;
        *(undefined1 *)0x799f = 1;
        FUN_1000_165a();
        uVar3 = extraout_AH_01;
      }
      if (*(int *)0x2078 != 0) {
        FUN_1000_3d46(&local_e,unaff_SS,CONCAT11(uVar3,*(undefined1 *)(local_4 * 0xf + 0x661f)));
      }
    }
    local_8 = *(uint *)0x206a % *(uint *)0xc204 - *(uint *)0x2068 % *(uint *)0xc204 >> 1;
    local_a = *(uint *)0x2068 % *(uint *)0xc204 + local_8;
    local_c = *(uint *)0x206a % *(uint *)0xc204 - local_8;
    *(undefined2 *)0x2090 = *(undefined2 *)0xc204;
    FUN_1000_4d3c();
    if (*(char *)0x661e == '\0') {
      if (local_e < '{') {
        local_e = local_e + '\x04';
      }
      *(undefined1 *)0x208c = 1;
    }
    local_11 = *(byte *)(local_4 * 0xf + 0x661d);
    if (*(char *)0x661e == '\x01') {
      uVar4 = (int)local_e >> 0xf;
      if (((int)(((int)local_e ^ uVar4) - uVar4) < 10) &&
         (uVar4 = (int)local_d >> 0xf, (int)(((int)local_d ^ uVar4) - uVar4) < 0x1e)) {
        if ((local_a < *(uint *)0x2074) && ((local_11 & 2) == 0)) {
          *(undefined2 *)0x2090 = 0xffff;
          FUN_1000_4dd3();
          if (*(char *)0x661e == '\0') {
            local_d = -0xf;
            *(byte *)(local_4 * 0xf + 0x661d) = local_11 | 1;
          }
          else {
            *(byte *)(local_4 * 0xf + 0x661d) = local_11 & 0xfc;
          }
        }
        else if ((*(uint *)0x2072 < local_c) && ((local_11 & 1) == 0)) {
          *(undefined2 *)0x2090 = 1;
          FUN_1000_4dd3();
          if (*(char *)0x661e == '\0') {
            local_d = '\x0f';
            *(byte *)(local_4 * 0xf + 0x661d) = local_11 | 2;
          }
          else {
            *(byte *)(local_4 * 0xf + 0x661d) = local_11 & 0xfc;
          }
        }
        if ((*(uint *)0x2074 <= local_a) && (local_c <= *(uint *)0x2072)) {
          *(byte *)(local_4 * 0xf + 0x661d) = local_11 & 0xfc;
        }
      }
      if (local_d != '\0') {
        if (local_d < '\x01') {
          local_d = local_d + '\x01';
        }
        else {
          local_d = local_d + -1;
        }
      }
    }
    else {
      *(byte *)(local_4 * 0xf + 0x661d) = local_11 & 0xfc;
    }
    if (*(char *)0x208c == '\x01') {
      *(undefined1 *)(local_4 * 0xf + 0x661e) = 0;
    }
    pcVar1 = (char *)(local_4 * 0xf + 0x661e);
    *pcVar1 = *pcVar1 + '\x01';
    *(int *)0x2068 = *(int *)0x2068 << 1;
    *(int *)0x206c = *(int *)0x206c << 1;
    uVar4 = (int)local_e >> 0xf;
    uVar5 = (int)local_d >> 0xf;
    *(int *)0x2074 = (((int)local_d ^ uVar5) - uVar5) + (((int)local_e ^ uVar4) - uVar4);
    uVar4 = *(int *)(local_4 * 0xf + 0x661b) - *(int *)0x2074;
    uVar5 = (int)uVar4 >> 0xf;
    *(int *)0x2072 = (uVar4 ^ uVar5) - uVar5;
    if (*(uint *)0x2072 < 0x40) {
      iVar7 = local_4 * 0xf;
      _local_18 = (uint *)CONCAT22(unaff_DS,(undefined2 *)(iVar7 + 0x6611));
      *(char *)(iVar7 + 0x6618) = local_e;
      *(char *)(iVar7 + 0x6619) = local_f;
      *(char *)(iVar7 + 0x661a) = local_10;
      *(char *)(iVar7 + 0x6617) = local_d;
      *_local_18 = *(undefined2 *)0x2068;
      *(undefined2 *)(iVar7 + 0x6613) = *(undefined2 *)0x206c;
      *(undefined2 *)(iVar7 + 0x661b) = *(undefined2 *)0x2074;
      if (*(char *)(local_4 * 0xf + 0x661e) == '_') {
        *(int *)0x206a = *(int *)0x206a << 1;
        FUN_1000_508b(&stack0xfffe,local_4);
      }
      if (local_4 == 0) {
        FUN_1920_00e9();
      }
    }
    else {
      *(undefined2 *)0x2074 = 0xea74;
      *(undefined1 *)0x799f = 3;
      FUN_1000_165a();
      FUN_1000_4fda();
      iVar7 = FUN_1920_13a8(*(undefined2 *)0x205a);
      *(int *)0x2072 = (*(uint *)0x206c >> 1) - iVar7;
      local_11 = *(byte *)0x6c;
      FUN_1000_2f9f(5,8,0xb,local_11,0,0,*(uint *)0x2072 / *(uint *)0xc204 << 3,
                    *(uint *)0x2072 % *(uint *)0xc204 << 3);
      if (*(int *)0x2072 == 1) {
        FUN_1000_06ab(1,2,*(undefined1 *)0x6d,local_11,(uint)*(byte *)0x208d * 0x26 + 0x1bc4,
                      unaff_DS);
      }
      FUN_1000_508b(&stack0xfffe,local_4);
    }
    local_4 = local_4 + -1;
  } while (local_4 != 0);
  return;
}



// ================================================
// Function: FUN_1000_568a at 1000:568a
// ================================================

void __cdecl16near FUN_1000_568a(void)

{
  undefined1 *puVar1;
  uint uVar2;
  int iVar3;
  undefined1 *puVar4;
  undefined2 unaff_DS;
  
  uVar2 = *(uint *)0x2074;
  if (uVar2 < 0x65) {
    puVar4 = (undefined1 *)(*(int *)0x2072 + -0x31c0);
    for (; uVar2 != 0; uVar2 = uVar2 - 1) {
      puVar1 = puVar4;
      puVar4 = puVar4 + 1;
      *puVar1 = 0xe;
    }
    for (iVar3 = 100 - *(int *)0x2074; iVar3 != 0; iVar3 = iVar3 + -1) {
      puVar1 = puVar4;
      puVar4 = puVar4 + 1;
      *puVar1 = 1;
    }
  }
  return;
}



// ================================================
// Function: FUN_1000_56b6 at 1000:56b6
// ================================================

void __cdecl16near FUN_1000_56b6(void)

{
  byte bVar1;
  int iVar2;
  undefined2 uVar3;
  int iVar4;
  byte *pbVar5;
  undefined2 unaff_DS;
  
  *(undefined1 *)0x661e = 0;
  iVar4 = 4;
  iVar2 = *(int *)0x2072;
  *(undefined2 *)0x2072 = 0;
  uVar3 = *(undefined2 *)0xc1fe;
  pbVar5 = (byte *)*(undefined2 *)0x2074;
  do {
    bVar1 = *pbVar5;
    if (bVar1 == 0x75) {
      *(undefined2 *)0x2072 = pbVar5;
      *(char *)0x661e = *(char *)0x661e + -1;
      *(char *)0x661e = *(char *)0x661e + -1;
    }
    if (((iVar2 < iVar4) && (bVar1 != 0)) && (bVar1 < 0x4d)) {
      *(char *)0x661e = *(char *)0x661e - *(char *)0x208c;
    }
    if (iVar4 == 4) {
      pbVar5 = pbVar5 + 1;
    }
    else if (iVar4 == 3) {
      pbVar5 = pbVar5 + *(int *)0xc204;
    }
    else if (iVar4 == 2) {
      pbVar5 = pbVar5 + -1;
    }
    iVar4 = iVar4 + -1;
  } while (iVar4 != 0);
  return;
}



// ================================================
// Function: FUN_1000_5715 at 1000:5715
// ================================================

void FUN_1000_5715(byte param_1,char param_2,byte *param_3)

{
  FUN_1920_04df();
  *param_3 = *param_3 + param_2;
  if (param_1 < *param_3) {
    *param_3 = param_1;
  }
  return;
}



// ================================================
// Function: FUN_1000_5740 at 1000:5740
// ================================================

void FUN_1000_5740(uint param_1)

{
  undefined2 uVar1;
  char *pcVar2;
  uint uVar3;
  uint *puVar4;
  undefined2 unaff_DS;
  uint *local_e;
  uint local_8;
  int local_6;
  char *local_4;
  
  FUN_1920_04df();
  uVar1 = *(undefined2 *)0x2074;
  *(undefined2 *)0x2074 = 0x27;
  *(undefined1 *)0x799f = 6;
  FUN_1000_165a();
  *(undefined2 *)0x2074 = uVar1;
  local_8 = 0;
  if ((*(char *)0x79a7 != '\0') && (*(byte *)0x79a7 != 0)) {
    local_4 = (char *)0x1;
    while( true ) {
      if (*(uint *)((int)local_4 * 0xe + 0x77c4) == (param_1 & 0x7fff)) {
        local_8 = (uint)local_4;
      }
      if (local_4 == (char *)(uint)*(byte *)0x79a7) break;
      local_4 = (char *)((int)local_4 + 1);
    }
  }
  if (local_8 != 0) {
    puVar4 = (uint *)(local_8 * 0xe + 0x77c0);
    _local_e = (uint *)CONCAT22(unaff_DS,puVar4);
    pcVar2 = (char *)*(int *)0x78ba;
    if (pcVar2 != (char *)0x0) {
      local_4 = (char *)0x1;
      while( true ) {
        uVar3 = *(uint *)((int)local_4 << 1) & 0x7fff;
        if ((*_local_e <= uVar3) && (uVar3 <= *(uint *)(local_8 * 0xe + 0x77c2))) {
          local_6 = 1;
          while( true ) {
            if ((*(char *)((int)puVar4 + local_6 + 5) != '\0') &&
               (*local_4 == *(char *)((int)puVar4 + local_6 + 5))) {
              *local_4 = *(char *)((int)puVar4 + local_6 + 9);
            }
            if (local_6 == 4) break;
            local_6 = local_6 + 1;
          }
        }
        if (local_4 == pcVar2) break;
        local_4 = local_4 + 1;
      }
    }
  }
  return;
}



// ================================================
// Function: FUN_1000_5872 at 1000:5872
// ================================================

void FUN_1000_5872(int param_1,byte param_2)

{
  uint uVar1;
  int iVar2;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined2 local_4;
  
  FUN_1920_04df();
  local_4 = 1;
  if (0x80 < param_2) {
    param_2 = param_2 + 0x80;
    local_4 = -1;
  }
  iVar2 = (uint)param_2 * 0x10;
  if (*(char *)(iVar2 + 0x79ed) == -1) {
    *(undefined2 *)(param_1 + -0x2c) = *(undefined2 *)(iVar2 + 0x79f5);
    *(undefined2 *)(param_1 + -0x2e) = *(undefined2 *)(iVar2 + 0x79f7);
    *(undefined2 *)(param_1 + -0xc) = 0;
    *(undefined2 *)(param_1 + -0xe) = 0;
  }
  else {
    *(int *)(param_1 + -0xc) = *(int *)(param_1 + -0xc) + *(int *)(iVar2 + 0x79f5) * local_4;
    *(int *)(param_1 + -0xe) = *(int *)(param_1 + -0xe) + *(int *)(iVar2 + 0x79f7) * local_4;
    uVar1 = (int)*(uint *)(param_1 + -0xc) >> 0xf;
    if ((int)(uint)*(byte *)(iVar2 + 0x79ed) < (int)((*(uint *)(param_1 + -0xc) ^ uVar1) - uVar1)) {
      if (*(int *)(param_1 + -0xc) < 1) {
        *(int *)(param_1 + -0xc) = *(int *)(param_1 + -0xc) + (uint)*(byte *)(iVar2 + 0x79ed);
      }
      else {
        *(int *)(param_1 + -0xc) = *(int *)(param_1 + -0xc) - (uint)*(byte *)(iVar2 + 0x79ed);
      }
    }
    uVar1 = (int)*(uint *)(param_1 + -0xe) >> 0xf;
    if ((int)(uint)*(byte *)(iVar2 + 0x79ed) < (int)((*(uint *)(param_1 + -0xe) ^ uVar1) - uVar1)) {
      if (*(int *)(param_1 + -0xe) < 1) {
        *(int *)(param_1 + -0xe) = *(int *)(param_1 + -0xe) + (uint)*(byte *)(iVar2 + 0x79ed);
      }
      else {
        *(int *)(param_1 + -0xe) = *(int *)(param_1 + -0xe) - (uint)*(byte *)(iVar2 + 0x79ed);
      }
    }
  }
  return;
}



// ================================================
// Function: FUN_1000_5999 at 1000:5999
// ================================================

void FUN_1000_5999(int param_1,uint param_2)

{
  int iVar1;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  uint *local_8;
  int local_4;
  
  FUN_1920_04df();
  local_4 = 1;
  *(undefined1 *)0x1b7c = 0;
  *(undefined1 *)0x1b81 = 0;
  while( true ) {
    if ((int)(uint)*(byte *)0x79a5 < local_4) {
      return;
    }
    iVar1 = local_4 * 7;
    _local_8 = (uint *)CONCAT22(unaff_DS,(uint *)(iVar1 + 0x7717));
    if (*_local_8 == (param_2 & 0x7fff)) break;
    local_4 = local_4 + 1;
  }
  *(undefined2 *)(param_1 + -0x2c) = *(undefined2 *)(iVar1 + 0x7719);
  *(undefined2 *)(param_1 + -0x2e) = *(undefined2 *)(iVar1 + 0x771b);
  *(undefined1 *)(param_1 + -0x12) = *(undefined1 *)0x6c;
  *(undefined2 *)0x2074 = 0x1a;
  *(undefined1 *)0x799f = 4;
  FUN_1000_165a();
  FUN_1000_2f9f(5,8,0xb,*(undefined1 *)(param_1 + -0x12),0,0,*(undefined2 *)(param_1 + -0x2e),
                *(undefined2 *)(param_1 + -0x2c));
  if (*(int *)0x2072 != 1) {
    return;
  }
  FUN_1000_06ab(1,2,*(undefined1 *)0x6d,*(undefined1 *)(param_1 + -0x12),
                (uint)*(byte *)0x208d * 0x26 + 0x1bc4,unaff_DS);
  return;
}



// ================================================
// Function: FUN_1000_5a75 at 1000:5a75
// ================================================

void FUN_1000_5a75(int param_1,byte param_2)

{
  int iVar1;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  iVar1 = (uint)*(byte *)((int)*(undefined4 *)(param_1 + 4) + 1) * 8;
  *(undefined2 *)(iVar1 + -0x3ddc) = *(undefined2 *)((uint)param_2 * 4 + -0x3cdc);
  *(undefined1 *)(iVar1 + -0x3dde) = *(undefined1 *)((uint)param_2 * 4 + -0x3cde);
  *(undefined1 *)(iVar1 + -0x3ddd) = *(undefined1 *)((uint)param_2 * 4 + -0x3cdd);
  *(char *)((int)*(undefined4 *)(param_1 + 4) + 0x14) = '\x10' - *(char *)(iVar1 + -0x3ddd);
  return;
}



// ================================================
// Function: FUN_1000_5afd at 1000:5afd
// ================================================

void FUN_1000_5afd(void)

{
  char *pcVar1;
  undefined2 uVar2;
  uint uVar3;
  char cVar4;
  uint uVar5;
  uint *puVar6;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  *(undefined1 *)0x79a3 = 0;
  *(undefined2 *)0x2072 = 0;
  *(undefined1 *)0x661e = 0;
  pcVar1 = (char *)*(undefined2 *)0xc1e8;
  cVar4 = *pcVar1;
  if (('f' < cVar4) && (cVar4 < 's')) {
    *(char *)0x79a3 = cVar4;
    uVar5 = (uint)(byte)((cVar4 + -0x67) * '\x02');
    *(undefined2 *)0x2072 = *(undefined2 *)(uVar5 + 0x1a);
    *(int *)0x2074 = *(int *)0x2074 + *(int *)(uVar5 + 2);
    uVar2 = *(undefined2 *)0x206e;
    uVar5 = *(uint *)((int)pcVar1 * 2);
    puVar6 = (uint *)((int)pcVar1 * 2 - *(int *)0x2066);
    uVar3 = *puVar6;
    if ((uVar3 != 0) && (uVar3 < 0x8000)) {
      *(undefined1 *)0x661e = 1;
    }
    if (uVar5 < 0x8000) {
      cVar4 = '\0';
      *(undefined2 *)((int)puVar6 + *(int *)0x2066) = 0;
    }
    else {
      cVar4 = -1;
    }
    *pcVar1 = cVar4;
  }
  return;
}



// ================================================
// Function: FUN_1000_5b86 at 1000:5b86
// ================================================

void FUN_1000_5b86(int param_1)

{
  uint uVar1;
  undefined2 unaff_SS;
  
  FUN_1920_04df();
  uVar1 = (int)*(uint *)(param_1 + -0xc) >> 0xf;
  if ((int)((*(uint *)(param_1 + -0xc) ^ uVar1) - uVar1) < 0x2b) {
    *(undefined2 *)(param_1 + -0xc) = 0;
  }
  else if (*(int *)(param_1 + -0xc) < 0) {
    *(int *)(param_1 + -0xc) = *(int *)(param_1 + -0xc) + 0x2a;
  }
  else {
    *(int *)(param_1 + -0xc) = *(int *)(param_1 + -0xc) + -0x2a;
  }
  return;
}



// ================================================
// Function: FUN_1000_5bcc at 1000:5bcc
// ================================================

void FUN_1000_5bcc(int param_1)

{
  byte bVar1;
  uint uVar2;
  undefined4 uVar3;
  char cVar4;
  int iVar5;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined2 local_c;
  undefined2 local_4;
  
  FUN_1920_04df();
  uVar3 = *(undefined4 *)(*(int *)(param_1 + 4) + 4);
  uVar2 = *(uint *)((int)uVar3 + 0x12);
  bVar1 = *(byte *)0x208d;
  if (bVar1 != 0) {
    local_4 = 1;
    while( true ) {
      iVar5 = local_4 * 0x26;
      _local_c = (char *)CONCAT22(unaff_DS,(char *)(iVar5 + 0x1bae));
      if ((*_local_c == '\x1f') && (*(byte *)(iVar5 + 0x1bd3) == uVar2)) {
        *_local_c = '\x0e';
        cVar4 = FUN_1920_13a8(10);
        *(char *)(iVar5 + 0x1bb0) = cVar4 + '(';
        *(undefined1 *)(iVar5 + 0x1bc3) = 2;
        *(undefined1 *)(iVar5 + 0x1bc9) = 0;
      }
      if (local_4 == bVar1) break;
      local_4 = local_4 + 1;
    }
  }
  uVar3 = *(undefined4 *)(*(int *)(param_1 + 4) + 4);
  *(undefined1 *)((int)uVar3 + 0x15) = 2;
  *(undefined1 *)*(undefined4 *)(*(int *)(param_1 + 4) + 4) = 0xe;
  uVar3 = *(undefined4 *)(*(int *)(param_1 + 4) + 4);
  *(undefined1 *)((int)uVar3 + 2) = 0x3c;
  uVar3 = *(undefined4 *)(*(int *)(param_1 + 4) + 4);
  *(undefined1 *)((int)uVar3 + 0x1b) = 0;
  FUN_1000_5740(1000);
  *(undefined2 *)0x2074 = 0x3d;
  *(undefined1 *)0x799f = 0xc;
  FUN_1000_165a();
  return;
}



// ================================================
// Function: FUN_1000_5cb0 at 1000:5cb0
// ================================================

void FUN_1000_5cb0(int param_1)

{
  char *pcVar1;
  byte bVar2;
  uint uVar3;
  int iVar4;
  byte bVar5;
  uint uVar6;
  int iVar7;
  undefined2 uVar8;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  uint local_e;
  uint local_c;
  int local_8;
  
  FUN_1920_04df();
  uVar8 = (undefined2)((ulong)*(undefined4 *)(param_1 + 4) >> 0x10);
  iVar4 = (int)*(undefined4 *)(param_1 + 4);
  uVar3 = *(uint *)(iVar4 + 0xe);
  bVar5 = (byte)((uint)*(undefined2 *)(iVar4 + 0xe) >> 8);
  *(int *)(param_1 + -0x2a) =
       (*(uint *)(param_1 + -0x2c) >> 3) + (*(uint *)(param_1 + -0x2e) >> 3) * *(int *)0xc204;
  local_8 = *(int *)(param_1 + -0x2a) - *(int *)0xc204;
  local_c = 1;
  *(undefined1 *)(param_1 + -0x22) = 0;
  for (; local_c <= (uVar3 & 0xff); local_c = local_c + 1) {
    bVar2 = *(byte *)((int)*(undefined4 *)0xc1e0 + local_8);
    if ((bVar2 != 0) && (bVar2 < 0x4d)) {
      *(undefined1 *)(param_1 + -0x22) = 1;
      break;
    }
    local_8 = local_8 + 1;
  }
  local_8 = (uint)bVar5 * *(int *)0xc204 + *(int *)(param_1 + -0x2a);
  local_c = 1;
  *(undefined1 *)(param_1 + -0x21) = 0;
  for (; local_c <= (uVar3 & 0xff); local_c = local_c + 1) {
    bVar2 = *(byte *)((int)*(undefined4 *)0xc1e0 + local_8);
    if ((bVar2 != 0) && (bVar2 < 0x53)) {
      *(undefined1 *)(param_1 + -0x21) = 1;
      break;
    }
    local_8 = local_8 + 1;
  }
  local_8 = *(int *)(param_1 + -0x2a) + -1;
  local_c = 1;
  *(undefined1 *)(param_1 + -0x23) = 0;
  for (; local_c <= bVar5; local_c = local_c + 1) {
    bVar2 = *(byte *)((int)*(undefined4 *)0xc1e0 + local_8);
    if ((bVar2 != 0) && (bVar2 < 0x4d)) {
      *(undefined1 *)(param_1 + -0x23) = 1;
      break;
    }
    local_8 = local_8 + *(int *)0xc204;
  }
  local_8 = (uVar3 & 0xff) + *(int *)(param_1 + -0x2a);
  local_c = 1;
  *(undefined1 *)(param_1 + -0x24) = 0;
  do {
    if (bVar5 < local_c) {
LAB_1000_5e59:
      if (*(uint *)0x78c2 % 0x1d == 0) {
        uVar6 = FUN_1920_13a8(100);
        if ((0x46 < uVar6) && ((*(uint *)0x78c2 & 1) == 0)) {
          *(undefined2 *)0x2074 = 0x69;
          *(undefined1 *)0x799f = 4;
          FUN_1000_165a();
        }
        if (*(int *)(param_1 + -4) < 1) {
          iVar7 = FUN_1920_13a8(800);
          *(int *)(param_1 + -0xc) = -0x96 - iVar7;
        }
        else {
          iVar7 = FUN_1920_13a8(800);
          *(int *)(param_1 + -0xc) = iVar7 + 0x96;
        }
        if (*(char *)(param_1 + -0x21) != '\0') {
          iVar7 = FUN_1920_13a8(0x5dc);
          *(int *)(param_1 + -0xe) = -300 - iVar7;
        }
      }
      *(undefined1 *)0x661e = 0;
      local_8 = *(int *)(param_1 + -0x2a);
      for (local_c = 1; iVar7 = local_8, local_c <= bVar5; local_c = local_c + 1) {
        for (local_e = 1; local_e <= (uVar3 & 0xff); local_e = local_e + 2) {
          if (*(char *)((int)*(undefined4 *)0xc1e0 + local_8) == 'u') {
            *(char *)0x661e = *(char *)0x661e + '\x01';
            *(int *)0x2072 = local_8;
          }
          local_8 = local_8 + 2;
        }
        local_8 = iVar7 + *(int *)0xc204;
      }
      if ('\0' < *(char *)0x661e) {
        FUN_1000_3a56();
        if (1 < *(byte *)(*(int *)0x2074 + 0x78d5)) {
          *(char *)0x661e = *(char *)0x661e << 1;
        }
        if ((int)(uint)*(byte *)(iVar4 + 0x24) < (int)*(char *)0x661e) {
          pcVar1 = (char *)((int)*(undefined4 *)(param_1 + 4) + 2);
          *pcVar1 = *pcVar1 + -1;
        }
        *(char *)(iVar4 + 0x24) = *(char *)(iVar4 + 0x24) - *(char *)0x661e;
        FUN_1000_5a75(param_1,0x2f);
      }
      if (*(char *)(iVar4 + 2) == -1) {
        FUN_1000_5bcc(&stack0xfffe);
      }
      if (*(char *)(param_1 + -0x21) == '\0') {
        *(int *)(param_1 + -0xe) = *(int *)(param_1 + -0xe) + 0x40;
      }
      if (((*(char *)(param_1 + -0x22) != '\0') && (*(int *)(param_1 + -0xe) < 0)) ||
         ((*(char *)(param_1 + -0x21) != '\0' && (0 < *(int *)(param_1 + -0xe))))) {
        *(int *)(param_1 + -0xe) = -(*(int *)(param_1 + -0xe) / 2);
      }
      if (((*(char *)(param_1 + -0x23) != '\0') && (*(int *)(param_1 + -0xc) < 0)) ||
         ((*(char *)(param_1 + -0x24) != '\0' && (0 < *(int *)(param_1 + -0xc))))) {
        *(int *)(param_1 + -0xc) = -(*(int *)(param_1 + -0xc) / 2);
      }
      return;
    }
    bVar2 = *(byte *)((int)*(undefined4 *)0xc1e0 + local_8);
    if ((bVar2 != 0) && (bVar2 < 0x4d)) {
      *(undefined1 *)(param_1 + -0x24) = 1;
      goto LAB_1000_5e59;
    }
    local_8 = local_8 + *(int *)0xc204;
    local_c = local_c + 1;
  } while( true );
}



// ================================================
// Function: FUN_1000_6053 at 1000:6053
// ================================================

void FUN_1000_6053(byte *param_1)

{
  char *pcVar1;
  byte bVar2;
  byte bVar3;
  undefined4 uVar4;
  bool bVar5;
  bool bVar6;
  bool bVar7;
  bool bVar8;
  bool bVar9;
  undefined1 uVar10;
  char cVar11;
  uint uVar12;
  uint uVar13;
  uint uVar14;
  undefined1 extraout_AH;
  uint uVar15;
  undefined1 extraout_AH_00;
  int iVar16;
  int iVar17;
  int iVar18;
  undefined1 uVar19;
  byte *pbVar20;
  undefined1 *puVar21;
  undefined1 *puVar22;
  uint *puVar23;
  undefined2 uVar24;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  bool bVar25;
  bool bVar26;
  undefined2 uVar27;
  undefined2 uVar28;
  uint *local_3c;
  uint *local_38;
  byte local_33;
  uint local_30;
  int local_2e;
  undefined1 *local_2c;
  byte local_1f;
  byte local_1e;
  byte local_1d;
  byte local_1c;
  byte local_1b;
  byte local_1a;
  byte local_19;
  undefined1 local_18;
  byte local_17;
  byte local_14;
  byte local_13;
  byte local_12;
  byte local_11;
  int local_10;
  uint local_e;
  uint local_c;
  uint local_a;
  uint local_8;
  uint local_6;
  int local_4;
  
  local_4 = 0x1000;
  local_6 = 0x605e;
  FUN_1920_04df();
  uVar24 = (undefined2)((ulong)param_1 >> 0x10);
  pbVar20 = (byte *)param_1;
  bVar2 = pbVar20[1];
  _local_3c = (uint *)CONCAT22(uVar24,pbVar20 + 0x16);
  if ((pbVar20[0x1b] != 0) && (pbVar20[0x19] = pbVar20[0x19] + 1, pbVar20[0x1a] < pbVar20[0x19])) {
    pbVar20[0x19] = 0;
    *(byte *)_local_3c = *(byte *)_local_3c + pbVar20[0x1c];
    if (pbVar20[0x1b] == 2) {
      if ((pbVar20[0x18] <= *(byte *)_local_3c) || (*(byte *)_local_3c <= pbVar20[0x17])) {
        pbVar20[0x1c] = -pbVar20[0x1c];
      }
    }
    else if ((pbVar20[0x18] < *(byte *)_local_3c) &&
            (*(byte *)_local_3c = pbVar20[0x17], pbVar20[0x1b] == 3)) {
      FUN_1920_090e(7,pbVar20 + 0x16,uVar24,pbVar20 + 0x1d,uVar24);
    }
    *(undefined2 *)((uint)bVar2 * 8 + -0x3ddc) =
         *(undefined2 *)((uint)*(byte *)_local_3c * 4 + -0x3cdc);
  }
  local_33 = pbVar20[0x15];
  bVar3 = pbVar20[0x16];
  if (local_33 == 0) {
    *(undefined1 *)0x1b82 = *(undefined1 *)0x1b78;
    *(undefined1 *)0x1b83 = *(undefined1 *)0x1b79;
    *(undefined1 *)0x1b84 = *(undefined1 *)0x1b7a;
    *(undefined1 *)0x1b85 = *(undefined1 *)0x1b7b;
    *(undefined1 *)0x1b86 = *(undefined1 *)0x1b7c;
    local_17 = 0x12;
    local_18 = 0x13;
    local_19 = 1;
    local_1a = *(byte *)0x64;
    local_1b = *(byte *)0x65;
    local_1c = *(byte *)0x62;
    local_1d = *(byte *)0x63;
    _local_38 = (uint *)CONCAT22(unaff_DS,(uint *)0x785a);
    local_1e = *(byte *)0x79ae;
    local_1f = 1;
    *(undefined2 *)0x79bc = 0x79b0;
    *(undefined2 *)0x79be = unaff_DS;
  }
  else if (local_33 == 1) {
    *(undefined1 *)0x1b82 = *(undefined1 *)0x1b7d;
    *(undefined1 *)0x1b83 = *(undefined1 *)0x1b7e;
    *(undefined1 *)0x1b84 = *(undefined1 *)0x1b7f;
    *(undefined1 *)0x1b85 = *(undefined1 *)0x1b80;
    *(undefined1 *)0x1b86 = *(undefined1 *)0x1b81;
    local_33 = 0;
    local_17 = 0x25;
    local_18 = 0x26;
    local_19 = 0x14;
    local_1a = *(byte *)0x68;
    local_1b = *(byte *)0x69;
    local_1c = *(byte *)0x66;
    local_1d = *(byte *)0x67;
    _local_38 = (uint *)CONCAT22(unaff_DS,(uint *)0x7888);
    local_1e = *(byte *)0x79af;
    local_1f = 2;
    *(undefined2 *)0x79bc = 0x79b1;
    *(undefined2 *)0x79be = unaff_DS;
  }
  _local_3c = (uint *)CONCAT22(unaff_DS,(int *)((uint)bVar2 * 8 + -0x3de2));
  local_2e = *_local_3c;
  local_12 = pbVar20[10];
  local_13 = pbVar20[0xc];
  local_e = *(uint *)(pbVar20 + 6);
  local_10 = *(int *)(pbVar20 + 8);
  local_30 = *(int *)((uint)bVar2 * 8 + -0x3de0) - (int)(char)pbVar20[0x14];
  local_11 = *param_1;
  if (((local_11 != 0) && (local_11 < 9)) || (0x12 < local_11)) {
    if (*(char *)0x79e6 == '\x01') {
      _local_3c = (uint *)CONCAT22(unaff_DS,(int *)((uint)*(byte *)0x1b89 * 8 + -0x3de2));
      local_6 = *_local_3c - local_2e;
      local_8 = *(int *)((uint)*(byte *)0x1b89 * 8 + -0x3de0) - local_30;
    }
    else {
      local_6 = 6000;
    }
    if (*(char *)0x79e7 == '\x01') {
      _local_3c = (uint *)CONCAT22(unaff_DS,(int *)((uint)*(byte *)0x1baf * 8 + -0x3de2));
      local_a = *_local_3c - local_2e;
      local_c = *(int *)((uint)*(byte *)0x1baf * 8 + -0x3de0) - local_30;
    }
    else {
      local_a = 6000;
    }
    if (((local_11 == 0x1f) && (pbVar20[2] = 0xfa, (*(uint *)(pbVar20 + 0xe) & 0xff) != 0)) &&
       ((FUN_1000_5872(&stack0xfffe,*(uint *)(pbVar20 + 0xe) & 0xff),
        *(uint *)(pbVar20 + 0xe) >> 8 != 0 &&
        (FUN_1000_5872(&stack0xfffe,*(uint *)(pbVar20 + 0xe) >> 8),
        (*(uint *)(pbVar20 + 0x10) & 0xff) != 0)))) {
      FUN_1000_5872(&stack0xfffe,*(uint *)(pbVar20 + 0x10) & 0xff);
    }
    if (((int)((local_6 ^ (int)local_6 >> 0xf) - ((int)local_6 >> 0xf)) < 10) &&
       ((int)((local_8 ^ (int)local_8 >> 0xf) - ((int)local_8 >> 0xf)) < 10)) {
      if (((local_11 == 0) || (8 < local_11)) && (local_11 < 0x1e)) {
        if (*(char *)0x79ae == '\0') {
          *(char *)0x79ae = local_11 - 0x12;
          pbVar20[0x15] = 5;
          *param_1 = 0xb;
          FUN_1000_5a75(&stack0xfffe,*(undefined1 *)((uint)*(byte *)0x79ae * 2 + 0x42));
          pbVar20[2] = 0x1a;
          pbVar20[0x1b] = 0;
          local_e = 0;
          FUN_1920_13a8(3);
          local_10 = FUN_1920_092a();
        }
      }
      else {
        *(char *)0x79e8 = *(char *)0x79e8 + '\x01';
      }
    }
    if (((int)((local_a ^ (int)local_a >> 0xf) - ((int)local_a >> 0xf)) < 10) &&
       ((int)((local_c ^ (int)local_c >> 0xf) - ((int)local_c >> 0xf)) < 10)) {
      if (((local_11 == 0) || (8 < local_11)) && (local_11 < 0x1e)) {
        if (*(char *)0x79af == '\0') {
          *(char *)0x79af = local_11 - 0x12;
          pbVar20[0x15] = 5;
          *param_1 = 0xb;
          FUN_1000_5a75(&stack0xfffe,*(undefined1 *)((uint)*(byte *)0x79af * 2 + 0x42));
          pbVar20[2] = 0x1a;
          pbVar20[0x1b] = 0;
          local_e = 0;
          FUN_1920_13a8(3);
          local_10 = FUN_1920_092a();
        }
      }
      else {
        *(char *)0x79e9 = *(char *)0x79e9 + '\x01';
      }
    }
    uVar15 = (local_c ^ (int)local_c >> 0xf) - ((int)local_c >> 0xf);
    uVar12 = (local_a ^ (int)local_a >> 0xf) - ((int)local_a >> 0xf);
    iVar16 = ((int)uVar12 >> 0xf) + ((int)uVar15 >> 0xf) + (uint)CARRY2(uVar12,uVar15);
    uVar13 = (local_8 ^ (int)local_8 >> 0xf) - ((int)local_8 >> 0xf);
    uVar14 = (local_6 ^ (int)local_6 >> 0xf) - ((int)local_6 >> 0xf);
    iVar17 = ((int)uVar14 >> 0xf) + ((int)uVar13 >> 0xf) + (uint)CARRY2(uVar14,uVar13);
    if ((iVar16 < iVar17) || ((iVar16 <= iVar17 && (uVar12 + uVar15 < uVar14 + uVar13)))) {
      local_6 = local_a;
      local_8 = local_c;
    }
  }
  if (local_33 == 6) {
    FUN_1000_5cb0(&stack0xfffe);
  }
  else {
    uVar4 = *(undefined4 *)0xc1e0;
    local_2c = (undefined1 *)
               ((int)uVar4 + ((local_30 >> 3) - 1) * *(int *)0xc204 + ((local_2e + 4U >> 3) - 1));
    iVar17 = 0;
    iVar16 = 4;
    puVar22 = local_2c;
    do {
      iVar18 = 4;
      puVar21 = puVar22;
      do {
        *(undefined1 *)(iVar17 + 0x2048) = *puVar21;
        iVar17 = iVar17 + 1;
        puVar21 = puVar21 + 1;
        iVar18 = iVar18 + -1;
      } while (iVar18 != 0);
      puVar22 = puVar22 + *(int *)0xc204;
      iVar16 = iVar16 + -1;
    } while (iVar16 != 0);
    if (local_33 == 5) {
      pbVar20[2] = pbVar20[2] - ((byte)*(undefined2 *)0x78c2 & 1);
      if (pbVar20[2] == 0) {
        FUN_1000_3358(*(undefined2 *)0x2082);
        *(int *)0x2082 = *(int *)0x2082 + -1;
        if (local_11 != 10) {
          return;
        }
        *(char *)0x208e = *(char *)0x208e + -1;
        return;
      }
    }
    else {
      if (local_11 == 0xd) {
        if ((*(char *)0x2055 == '\0') || (0x52 < *(byte *)0x2055)) {
          bVar25 = false;
        }
        else {
          bVar25 = true;
        }
        if ((*(char *)0x2050 == '\0') || (0x4c < *(byte *)0x2050)) {
          bVar26 = false;
        }
        else {
          bVar26 = true;
        }
        if ((*(char *)0x2052 == '\0') || (0x4c < *(byte *)0x2052)) {
          bVar9 = false;
        }
        else {
          bVar9 = true;
        }
        if ((*(char *)0x204d == '\0') || (0x4c < *(byte *)0x204d)) {
          bVar5 = false;
        }
        else {
          bVar5 = true;
        }
      }
      else {
        if ((*(char *)0x2055 == '\0') || (0x52 < *(byte *)0x2055)) {
          if ((*(char *)0x2056 == '\0') || (0x52 < *(byte *)0x2056)) {
            bVar25 = false;
          }
          else {
            bVar25 = true;
          }
        }
        else {
          bVar25 = true;
        }
        if ((*(char *)0x204c == '\0') || (0x4c < *(byte *)0x204c)) {
          if ((*(char *)0x2050 == '\0') || (0x4c < *(byte *)0x2050)) {
            bVar26 = false;
          }
          else {
            bVar26 = true;
          }
        }
        else {
          bVar26 = true;
        }
        if ((*(char *)0x204f == '\0') || (0x4c < *(byte *)0x204f)) {
          if ((*(char *)0x2053 == '\0') || (0x4c < *(byte *)0x2053)) {
            bVar9 = false;
          }
          else {
            bVar9 = true;
          }
        }
        else {
          bVar9 = true;
        }
        if ((*(char *)0x2049 == '\0') || (0x4c < *(byte *)0x2049)) {
          if ((*(char *)0x204a == '\0') || (0x4c < *(byte *)0x204a)) {
            bVar5 = false;
          }
          else {
            bVar5 = true;
          }
        }
        else {
          bVar5 = true;
        }
      }
      if (local_33 == 0) {
        if ((bVar26) && ((*(char *)0x204c == '\0' || (0x4c < *(byte *)0x204c)))) {
          bVar6 = true;
        }
        else {
          bVar6 = false;
        }
        if ((bVar9) && ((*(char *)0x204f == '\0' || (0x4c < *(byte *)0x204f)))) {
          bVar7 = true;
        }
        else {
          bVar7 = false;
        }
        if ((bVar25) && (-1 < local_10)) {
          if (0 < local_10) {
            local_30 = local_30 & 0xfff8;
            if (local_10 < 0x641) {
              local_10 = 0;
            }
            else {
              uVar10 = (undefined1)(local_30 >> 8);
              if (pbVar20[0x17] != local_17) {
                if (pbVar20[0x1b] == 0) {
                  FUN_1000_06ab(3,2,CONCAT11(uVar10,local_19),CONCAT11(uVar10,local_19),
                                pbVar20 + 0x1d,uVar24);
                }
                else {
                  FUN_1920_090e(7,pbVar20 + 0x1d,uVar24,pbVar20 + 0x16,uVar24);
                }
                FUN_1000_06ab(3,3,CONCAT11((char)(local_17 - 1 >> 8),local_17),local_17 - 1,
                              pbVar20 + 0x16,uVar24);
                *(undefined2 *)((uint)bVar2 * 8 + -0x3ddc) =
                     *(undefined2 *)((uint)local_17 * 4 + -0x3cdc);
              }
              local_10 = -local_10 / 4;
            }
          }
        }
        else {
          local_10 = local_10 + 0x40;
          if (0x7ff < local_10) {
            local_10 = 0x7ff;
          }
        }
        if ((uint)*(byte *)0x1b83 + (uint)*(byte *)0x1b84 == 2) {
          *(undefined1 *)0x1b83 = 0;
          *(undefined1 *)0x1b84 = 0;
          *(char *)*(undefined4 *)0x79bc = *(char *)*(undefined4 *)0x79bc + '\x01';
        }
        else {
          if (4 < *(byte *)*(undefined4 *)0x79bc) {
            *(undefined2 *)0x2074 = 0x24;
            *(undefined1 *)0x799f = 2;
            FUN_1000_165a();
            *(undefined1 *)(local_1f + 0x1b75) = 2;
            *(undefined1 *)0x79a3 = 0;
            do {
              iVar16 = (uint)*(byte *)(local_1f + 0x1b73) % 4 + 1;
              local_14 = (byte)iVar16;
              *(byte *)(local_1f + 0x1b73) = local_14;
              *(char *)0x79a3 = *(char *)0x79a3 + '\x01';
              if (*(char *)((uint)local_1f * 4 + iVar16 + 0x1b67) != '\0') break;
            } while (*(byte *)0x79a3 < 5);
            if (4 < *(byte *)0x79a3) {
              pbVar20[0x24] = 0xfe;
            }
          }
          *(undefined1 *)*(undefined4 *)0x79bc = 0;
          uVar15 = (uint)*(byte *)0x1b86 + (uint)*(byte *)0x1b82;
          if (uVar15 == 2) {
            *(undefined1 *)0x1b86 = 0;
            *(undefined1 *)0x1b82 = 0;
          }
          if (*(char *)0x1b86 == '\x01') {
            if (*(char *)0x2055 == 'E') {
              uVar15 = FUN_1000_5999(&stack0xfffe,
                                     *(undefined2 *)
                                      ((int)*(undefined4 *)0x6612 +
                                      (int)(local_2c + *(int *)0xc204 * 3 + 1) * 2));
            }
            else if ((*(char *)0x2055 != '\'') || (bVar5)) {
              if ((bVar25) && ((local_10 == 0 && (*(int *)(pbVar20 + 0xe) == 0)))) {
                if ((*(char *)0x2055 == '\0') || (0x4c < *(byte *)0x2055)) {
                  if ((*(char *)0x2056 == '\0') || (0x4c < *(byte *)0x2056)) {
                    bVar8 = false;
                  }
                  else {
                    bVar8 = true;
                  }
                }
                else {
                  bVar8 = true;
                }
                if (bVar8) {
                  *(undefined1 *)0x1b86 = 0;
                }
                else {
                  uVar10 = (undefined1)(uVar15 >> 8);
                  uVar15 = CONCAT11(uVar10,pbVar20[0x17]);
                  if (pbVar20[0x17] != local_17) {
                    if (pbVar20[0x1b] == 0) {
                      FUN_1000_06ab(3,2,CONCAT11(uVar10,local_19),CONCAT11(uVar10,local_19),
                                    pbVar20 + 0x1d,uVar24);
                    }
                    else {
                      FUN_1920_090e(7,pbVar20 + 0x1d,uVar24,pbVar20 + 0x16,uVar24);
                    }
                    FUN_1000_06ab(3,3,CONCAT11((char)(local_17 - 1 >> 8),local_18),local_17 - 1,
                                  pbVar20 + 0x16,uVar24);
                    uVar15 = (uint)bVar2;
                    *(undefined2 *)(uVar15 * 8 + -0x3ddc) =
                         *(undefined2 *)((uint)local_17 * 4 + -0x3cdc);
                  }
                  (pbVar20 + 0xe)[0] = 4;
                  (pbVar20 + 0xe)[1] = 0;
                }
              }
              else {
                *(undefined1 *)0x1b86 = 0;
              }
            }
            else {
              local_10 = -2000;
              *(undefined2 *)0x2074 = 0x35;
              *(undefined1 *)0x799f = 5;
              FUN_1000_165a();
              uVar15 = FUN_1000_2f9f(5,5,0xb,0x5b,0xff38,0,local_30 + 0xd,local_2e + 4);
              if (*(int *)0x2072 == 1) {
                uVar15 = (uint)*(byte *)0x208d;
                *(undefined1 *)(uVar15 * 0x26 + 0x1bc9) = 0;
              }
            }
          }
          uVar10 = (undefined1)(uVar15 >> 8);
          if (*(int *)(pbVar20 + 0xe) != 0) {
            local_30 = local_30 + 2;
            *(int *)(pbVar20 + 0xe) = *(int *)(pbVar20 + 0xe) + -1;
            pbVar20[2] = 0xf8;
          }
          if (*(char *)0x1b83 == '\x01') {
            if (bVar6) {
              local_10 = -500;
              bVar26 = false;
              local_e = 0xff06;
            }
            if ((bVar3 < local_1a) || (local_1b < bVar3)) {
              FUN_1000_06ab(1,0,CONCAT11(uVar10,local_1b),CONCAT11(uVar10,local_1a),pbVar20 + 0x16,
                            uVar24);
              pbVar20[2] = 0;
              uVar10 = extraout_AH;
            }
            if (-0x400 < (int)local_e) {
              local_e = local_e - 0x40;
            }
          }
          if (*(char *)0x1b84 == '\x01') {
            if (bVar7) {
              local_10 = -500;
              bVar9 = false;
              local_e = 0xfa;
            }
            if ((local_1d < bVar3) || (bVar3 < local_1c)) {
              FUN_1000_06ab(1,0,CONCAT11(uVar10,local_1d),CONCAT11(uVar10,local_1c),pbVar20 + 0x16,
                            uVar24);
              pbVar20[2] = 0;
            }
            if ((int)local_e < 0x400) {
              local_e = local_e + 0x40;
            }
          }
        }
        if (bVar25) {
          if ((*(char *)0x1b82 == '\x01') && (local_10 == 0)) {
            local_10 = -0x350;
          }
          if ((uint)*(byte *)0x1b84 + (uint)*(byte *)0x1b83 == 0) {
            FUN_1000_5b86(&stack0xfffe);
          }
        }
        pbVar20[0x1a] =
             (byte)(4 - (int)((local_e ^ (int)local_e >> 0xf) - ((int)local_e >> 0xf)) / 0x100);
        if (((uint)*(byte *)0x1b83 + (uint)*(byte *)0x1b84 + (uint)*(byte *)0x1b86 == 0) &&
           (pbVar20[2] = pbVar20[2] + 1, pbVar20[2] == 5)) {
          _local_3c = (uint *)CONCAT22(uVar24,pbVar20 + 0x16);
          pbVar20[0x1b] = 0;
          *(byte *)_local_3c = 1;
          *(undefined2 *)((uint)bVar2 * 8 + -0x3ddc) = *(undefined2 *)((uint)local_19 * 4 + -0x3cdc)
          ;
        }
        if ((*(char *)0x1b85 == '\x01') &&
           (local_14 = *(byte *)(local_1f + 0x1b73),
           *(char *)((uint)local_1f * 4 + (uint)local_14 + 0x1b67) != '\0')) {
          if (local_14 == 4) {
            *(undefined1 *)0x79a3 = 200;
          }
          else {
            *(char *)0x79a3 = local_14 * '\n' + '\n';
          }
          FUN_1000_2f9f(2,CONCAT11((char)(local_14 + 0xc >> 8),*(undefined1 *)0x79a3),local_14 + 0xc
                        ,local_14 + 0x39,local_10 + -500,(int)(local_e * 3) / 2,local_30,local_2e);
          if (*(int *)0x2072 == 1) {
            FUN_1000_06ab(0,0,0,0,(uint)*(byte *)0x208d * 0x26 + 0x1bc4,unaff_DS);
            pcVar1 = (char *)((uint)local_1f * 4 + (uint)*(byte *)(local_1f + 0x1b73) + 0x1b67);
            *pcVar1 = *pcVar1 + -1;
            *(undefined1 *)(local_1f + 0x1b75) = 1;
          }
          *(undefined1 *)0x1b7b = 0;
          *(undefined1 *)0x1b80 = 0;
        }
        *(undefined2 *)0x2074 = 0;
        *(undefined2 *)0xc1e8 = local_2c + *(int *)0xc204;
        *(undefined1 *)0x79ab = 0;
        local_4 = 1;
        while( true ) {
          if (local_4 == 1) {
            *(int *)0xc1e8 = *(int *)0xc1e8 + 1;
            *(undefined1 *)0x78d2 = 0xfe;
            *(undefined1 *)0x78d4 = 0xfe;
          }
          else if (local_4 == 2) {
            *(int *)0xc1e8 = *(int *)0xc1e8 + 1;
            *(undefined1 *)0x78d2 = 10;
            *(undefined1 *)0x78d4 = 0xfe;
          }
          else if (local_4 == 3) {
            *(int *)0xc1e8 = *(int *)0xc1e8 + *(int *)0xc204;
            *(undefined1 *)0x78d2 = 10;
            *(undefined1 *)0x78d4 = 10;
          }
          else if (local_4 == 4) {
            *(int *)0xc1e8 = *(int *)0xc1e8 + -1;
            *(undefined1 *)0x78d2 = 0xfe;
            *(undefined1 *)0x78d4 = 10;
          }
          if (*(char *)((int)*(undefined4 *)0xc1e0 + *(int *)0xc1e8) == 'r') {
            FUN_1000_5740(*(undefined2 *)((int)*(undefined4 *)0x6612 + *(int *)0xc1e8 * 2));
          }
          FUN_1000_5afd(&stack0xfffe);
          if (*(char *)0x661e == '\x01') {
            FUN_1000_370e(&local_14,unaff_SS,0,0,*(int *)0xc1e8 - *(int *)0xc204);
          }
          if (*(char *)0x79a3 == *(char *)0x79b4) {
            *(int *)0x2088 = *(int *)0x2088 + 1;
          }
          if (0x6c < *(byte *)0x79a3) {
            *(undefined1 *)0x79ab = 1;
          }
          if ((*(int *)0x2072 != 0) && (*(byte *)0x208e < 0xe)) {
            iVar16 = local_2e + *(char *)0x78d2;
            iVar17 = local_30 + (int)*(char *)0x78d4;
            uVar27 = 0;
            iVar18 = FUN_1920_13a8(200);
            FUN_1000_2f9f(5,0xc,10,*(undefined2 *)0x2072,-0x28 - iVar18,uVar27,iVar17,iVar16);
            FUN_1000_06ab(0,0,0,0,(uint)*(byte *)0x208d * 0x26 + 0x1bc4,unaff_DS);
            if (*(int *)0x2072 == 1) {
              *(char *)0x208e = *(char *)0x208e + '\x01';
            }
          }
          if (local_4 == 4) break;
          local_4 = local_4 + 1;
        }
        puVar23 = (uint *)_local_38;
        uVar27 = (undefined2)((ulong)_local_38 >> 0x10);
        if (*(int *)0x2074 != 0) {
          *(undefined1 *)(puVar23 + 0x16) = 0;
          uVar12 = *(uint *)0x2074;
          uVar15 = *_local_38;
          *_local_38 = *_local_38 + uVar12;
          puVar23[1] = puVar23[1] + (uint)CARRY2(uVar15,uVar12);
          *(undefined2 *)0x2074 = 0;
          if (*(char *)0x79ab != '\0') {
            *(undefined2 *)0x2074 = 0x12;
          }
          *(undefined1 *)0x799f = 3;
          FUN_1000_165a();
        }
        if (local_1e != 0) {
          if (local_1e == 2) {
            pbVar20[0x24] = 100;
          }
          else if (local_1e == 3) {
            pbVar20[0x24] = pbVar20[0x24] + 0x21;
            if (100 < pbVar20[0x24]) {
              pbVar20[0x24] = 100;
            }
          }
          else if (local_1e == 4) {
            *(undefined1 *)0x79b2 = 0x2e;
          }
          else if (local_1e == 5) {
            *(undefined1 *)((uint)local_1f * 4 + 0x1b68) = 200;
            iVar17 = (uint)local_1f * 4 + 0x1b69;
            uVar28 = unaff_DS;
            iVar16 = FUN_1920_13a8(10);
            FUN_1000_5715(99,iVar16 + 1,iVar17,uVar28);
            iVar17 = (uint)local_1f * 4 + 0x1b6a;
            uVar28 = unaff_DS;
            iVar16 = FUN_1920_13a8(4);
            FUN_1000_5715(99,iVar16 + 1,iVar17,uVar28);
            *(undefined1 *)(local_1f + 0x1b75) = 1;
          }
          else if (local_1e == 6) {
            *(undefined1 *)((uint)local_1f * 4 + 0x1b68) = 200;
            iVar17 = (uint)local_1f * 4 + 0x1b69;
            uVar28 = unaff_DS;
            iVar16 = FUN_1920_13a8(0xd);
            FUN_1000_5715(99,iVar16 + 1,iVar17,uVar28);
            iVar17 = (uint)local_1f * 4 + 0x1b6a;
            uVar28 = unaff_DS;
            iVar16 = FUN_1920_13a8(5);
            FUN_1000_5715(99,iVar16 + 2,iVar17,uVar28);
            iVar17 = (uint)local_1f * 4 + 0x1b6b;
            uVar28 = unaff_DS;
            iVar16 = FUN_1920_13a8(2);
            FUN_1000_5715(99,iVar16 + 1,iVar17,uVar28);
            *(undefined1 *)(local_1f + 0x1b75) = 1;
          }
          *(undefined1 *)(puVar23 + 0x16) = 0;
          uVar12 = *(uint *)((uint)local_1e * 2 + 0x34);
          uVar15 = *_local_38;
          *_local_38 = *_local_38 + uVar12;
          puVar23[1] = puVar23[1] + (uint)CARRY2(uVar15,uVar12);
          *(undefined2 *)0x2074 = 8;
          *(undefined1 *)0x799f = 5;
          FUN_1000_165a();
        }
        *(undefined2 *)0x2074 = local_2c + *(int *)0xc204 + 1;
        *(undefined2 *)0x2072 = 2;
        *(undefined1 *)0x208c = 2;
        FUN_1000_56b6();
        if (*(int *)0x2072 != 0) {
          FUN_1000_3a56();
          local_e = (int)*(char *)(*(int *)0x2074 * 0xb + 0x2097) << 3;
          local_10 = (int)*(char *)(*(int *)0x2074 * 0xb + 0x2098) << 3;
          if (1 < *(byte *)(*(int *)0x2074 + 0x78d5)) {
            *(char *)0x661e =
                 *(char *)0x661e -
                 (char)((ulong)(long)(int)(uint)*(byte *)(*(int *)0x2074 + 0x78d5) / 10);
          }
        }
        pbVar20[0x24] = pbVar20[0x24] + *(char *)0x661e;
      }
      else if (local_33 == 2) {
        if ((bVar25) && (-1 < local_10)) {
          if (0 < local_10) {
            local_10 = 0;
            local_30 = local_30 & 0xfff8;
          }
        }
        else {
          local_10 = local_10 + 0x40;
          if (0x7ff < local_10) {
            local_10 = 0x7ff;
          }
        }
        if (bVar25) {
          FUN_1000_5b86(&stack0xfffe);
        }
      }
      else if (local_33 == 4) {
        if ((*(char *)0x2055 == '\0') || (0x4c < *(byte *)0x2055)) {
          if ((*(char *)0x2056 == '\0') || (0x4c < *(byte *)0x2056)) {
            bVar25 = false;
          }
          else {
            bVar25 = true;
          }
        }
        else {
          bVar25 = true;
        }
        if ((bVar25) && (bVar5)) {
          local_10 = 0;
        }
        if ((bVar25) && (0 < local_10)) {
          local_10 = -local_10 / 2;
        }
        *(undefined2 *)0xc1e8 = *(undefined2 *)(pbVar20 + 0xe);
        *(undefined2 *)0x2074 = *(undefined2 *)(pbVar20 + 0x10);
        *(undefined2 *)0x2072 = *(undefined2 *)(pbVar20 + 0x12);
        if (*(uint *)0x78c2 % *(uint *)0xc1e8 == 0) {
          uVar15 = ((local_6 ^ (int)local_6 >> 0xf) - ((int)local_6 >> 0xf)) +
                   ((local_8 ^ (int)local_8 >> 0xf) - ((int)local_8 >> 0xf));
          iVar16 = (int)uVar15 >> 0xf;
          if ((iVar16 < 0) || ((iVar16 < 1 && (uVar15 < *(uint *)0x2072)))) {
            FUN_1000_346b(&local_10,unaff_SS,&local_e,unaff_SS,0x2074,unaff_DS,&local_8,unaff_SS,
                          &local_6,unaff_SS);
          }
          else {
            iVar16 = FUN_1920_13a8(*(int *)0x2074 << 1);
            local_e = iVar16 - *(int *)0x2074;
            local_10 = FUN_1920_13a8(*(int *)0x2074 << 1);
            local_10 = local_10 - *(int *)0x2074;
          }
        }
      }
      else if (local_33 == 3) {
        if ((bVar26) || (bVar9)) {
          bVar6 = true;
        }
        else {
          bVar6 = false;
        }
        if ((bVar25) && (-1 < local_10)) {
          if (0 < local_10) {
            local_10 = 0;
            local_30 = local_30 & 0xfff8;
            bVar6 = true;
          }
        }
        else {
          local_10 = local_10 + 0x40;
          if (0x7ff < local_10) {
            local_10 = 0x7ff;
          }
        }
        if (bVar25) {
          if (local_e == 0) {
            local_e = *(uint *)(pbVar20 + 0xe);
          }
          else {
            iVar16 = (local_e ^ (int)local_e >> 0xf) - ((int)local_e >> 0xf);
            if ((iVar16 < 0) || (iVar16 != *(int *)(pbVar20 + 0xe))) {
              local_e = FUN_1920_092a();
              bVar6 = true;
            }
          }
          if ((*(char *)0x2054 == '\0') || (0x52 < *(byte *)0x2054)) {
            bVar25 = true;
          }
          else {
            bVar25 = false;
          }
          if ((*(char *)0x2057 == '\0') || (0x52 < *(byte *)0x2057)) {
            bVar7 = true;
          }
          else {
            bVar7 = false;
          }
          if ((bVar25) || (bVar7)) {
            bVar6 = true;
          }
          if ((!bVar25) || (!bVar7)) {
            if ((int)local_e < 0) {
              if (bVar25) {
                local_e = -local_e;
              }
            }
            else if (bVar7) {
              local_e = -local_e;
            }
          }
        }
        if (bVar6) {
          if ((int)local_e < 0) {
            local_14 = pbVar20[3];
            _local_3c = (uint *)CONCAT22(uVar24,pbVar20 + 0x16);
            pbVar20[0x17] = *(byte *)((uint)local_14 * 2 + 0x58);
            pbVar20[0x18] = *(byte *)((uint)local_14 * 2 + 0x59);
            *(byte *)_local_3c = pbVar20[0x17];
          }
          else if (0 < (int)local_e) {
            local_14 = pbVar20[4];
            _local_3c = (uint *)CONCAT22(uVar24,pbVar20 + 0x16);
            pbVar20[0x17] = *(byte *)((uint)local_14 * 2 + 0x58);
            pbVar20[0x18] = *(byte *)((uint)local_14 * 2 + 0x59);
            *(byte *)_local_3c = pbVar20[0x17];
          }
        }
      }
      if (local_11 == 4) {
        *(undefined2 *)0xc1e8 = local_2c + *(int *)0xc204;
        *(undefined2 *)0x2074 = 0;
        FUN_1000_5afd(&stack0xfffe);
        *(int *)0x2074 = *(int *)0x2074 + *(int *)0x2072;
        if (*(char *)0x661e == '\x01') {
          FUN_1000_370e(&local_14,unaff_SS,0,0,*(int *)0xc1e8 - *(int *)0xc204);
        }
        *(int *)0xc1e8 = *(int *)0xc1e8 + *(int *)0xc204 + 1;
        FUN_1000_5afd(&stack0xfffe);
        *(int *)0x2074 = *(int *)0x2074 + *(int *)0x2072;
        if (*(int *)0x2074 != 0) {
          *(undefined1 *)0x799f = 1;
          *(undefined2 *)0x2074 = 0x21;
          FUN_1000_165a();
        }
      }
      if ((bVar5) && (local_10 < 0)) {
        local_10 = 1;
      }
      if ((bVar26) && (bVar9)) {
        local_e = 0;
      }
      if (((bVar26) && ((int)local_e < 0)) || ((bVar9 && (0 < (int)local_e)))) {
        local_e = (int)-local_e / 2;
        if ((int)local_e < 0) {
          local_2e = local_2e + -1;
        }
        else {
          local_2e = local_2e + 1;
        }
      }
    }
  }
  uVar10 = 0;
  if (local_10 < 0) {
    uVar10 = 0xff;
  }
  bVar25 = CARRY1(local_13,(byte)local_10);
  local_13 = local_13 + (byte)local_10;
  uVar15 = (uint)local_10 >> 8;
  uVar19 = 0;
  if ((int)local_e < 0) {
    uVar19 = 0xff;
  }
  bVar26 = CARRY1(local_12,(byte)local_e);
  local_12 = local_12 + (byte)local_e;
  uVar12 = local_2e + CONCAT11(uVar19,(char)(local_e >> 8)) + (uint)bVar26;
  if (((local_11 != 0) && (local_11 < 9)) && (local_11 != 0x1f)) {
    *(undefined2 *)0x2072 = 0;
    *(undefined2 *)0x2074 = local_2c + *(int *)0xc204 + 1;
    *(undefined1 *)0x208c = 1;
    FUN_1000_56b6();
    if (*(char *)0x661e != '\0') {
      if ((int)local_e < 1) {
        local_14 = 1;
      }
      else {
        local_14 = 2;
      }
      FUN_1000_5a75(&stack0xfffe,*(undefined1 *)((uint)local_11 * 2 + (uint)local_14 + 0x77));
      pbVar20[0x19] = pbVar20[0x1a] - 4;
      if ((int)((uint)pbVar20[0x24] + (int)*(char *)0x661e) < 0) {
        pbVar20[0x1b] = 0;
        pbVar20[0x15] = 2;
        *param_1 = 0xc;
        pbVar20[2] = 0x19;
        if (pbVar20[0x25] != 0) {
          pcVar1 = (char *)((uint)pbVar20[0x25] * 0x1e + 0x74b2);
          *pcVar1 = *pcVar1 + '\x01';
        }
      }
      else {
        pbVar20[0x24] = pbVar20[0x24] + *(char *)0x661e;
      }
    }
  }
  uVar15 = local_30 + CONCAT11(uVar10,(char)uVar15) + (uint)bVar25 + (int)(char)pbVar20[0x14];
  _local_3c = (uint *)CONCAT22(unaff_DS,(uint *)((uint)bVar2 * 8 + -0x3de2));
  *_local_3c = uVar12;
  *(uint *)((uint)bVar2 * 8 + -0x3de0) = uVar15;
  *(uint *)(pbVar20 + 10) = (uint)local_12;
  *(uint *)(pbVar20 + 0xc) = (uint)local_13;
  *(uint *)(pbVar20 + 6) = local_e;
  *(int *)(pbVar20 + 8) = local_10;
  if (local_11 < 0xc) {
    return;
  }
  if (local_33 == 5) {
    return;
  }
  if (0x1d < local_11) {
    return;
  }
  pbVar20[2] = pbVar20[2] - ((byte)*(undefined2 *)0x78c2 & 1);
  if ((pbVar20[2] != 0) && (pbVar20[2] != 0xff)) {
    return;
  }
  if ((0xc < local_11) && (local_11 < 0x13)) {
    FUN_1000_414a(CONCAT11((char)(local_11 - 0xc >> 8),*(undefined1 *)(local_11 + 0x25)),
                  (uVar12 >> 3) + (uVar15 >> 3) * *(int *)0xc204);
  }
  if (local_11 == 0xc) {
    uVar10 = FUN_1920_13a8(100);
    *(undefined1 *)0x79a3 = uVar10;
    iVar16 = FUN_1920_13a8(0x14);
    *(int *)0x2074 = iVar16 + -0x158c;
    *(undefined1 *)0x799f = 4;
    FUN_1000_165a();
    if (*(byte *)0x52 <= *(byte *)0x79a3) {
      local_14 = 1;
      while (*(byte *)(local_14 + 0x52) < *(byte *)0x79a3) {
        local_14 = local_14 + 1;
      }
      local_14 = local_14 - 1;
      FUN_1000_5a75(&stack0xfffe,local_14 + 0x3e);
      *param_1 = local_14 + 0x13;
      pbVar20[2] = 100;
      pbVar20[0x1b] = 0;
      *(int *)(pbVar20 + 8) = *(int *)(pbVar20 + 8) + -200;
      goto LAB_1000_76f4;
    }
  }
  pbVar20[0x15] = 5;
  (pbVar20 + 6)[0] = 0;
  (pbVar20 + 6)[1] = 0;
  (pbVar20 + 8)[0] = 0;
  (pbVar20 + 8)[1] = 0;
  *param_1 = 0;
  pbVar20[2] = 0x12;
  if (local_11 < 0x13) {
    local_14 = *(byte *)0x6a;
  }
  else {
    local_14 = *(byte *)0x6c;
  }
  FUN_1000_5a75(&stack0xfffe,local_14);
  FUN_1000_06ab(1,2,CONCAT11(extraout_AH_00,*(undefined1 *)0x6d),CONCAT11(extraout_AH_00,local_14),
                pbVar20 + 0x16,uVar24);
LAB_1000_76f4:
  if (local_11 < 0x13) {
    local_14 = local_11 - 0xc;
    if (local_14 == 0) {
      local_14 = 1;
    }
    cVar11 = local_14 + 1;
    if (local_14 != 0xff) {
      local_11 = '\x01';
      while( true ) {
        uVar13 = uVar15;
        uVar14 = uVar12;
        iVar16 = FUN_1920_13a8(600);
        iVar16 = iVar16 + -300;
        iVar17 = FUN_1920_13a8(600);
        FUN_1000_2f9f(5,0xf,0xb,0xd,iVar17 + -300,iVar16,uVar13,uVar14);
        if (*(int *)0x2072 == 1) {
          FUN_1000_06ab(2,2,*(undefined1 *)0x6d,*(undefined1 *)0x6a,
                        (uint)*(byte *)0x208d * 0x26 + 0x1bc4,unaff_DS);
        }
        if (local_11 == cVar11) break;
        local_11 = local_11 + '\x01';
      }
    }
  }
  return;
}



// ================================================
// Function: entry at 1000:7783
// ================================================

/* WARNING: Stack frame is not setup normally: Input value of stackpointer is not used */
/* WARNING: This function may have set the stack pointer */

void entry(void)

{
  uint *puVar1;
  char *pcVar2;
  bool bVar3;
  code *pcVar4;
  undefined4 uVar5;
  undefined2 *puVar6;
  undefined1 uVar7;
  char cVar8;
  uint uVar9;
  undefined2 uVar10;
  int iVar11;
  undefined2 extraout_DX;
  uint in_BX;
  undefined1 *puVar12;
  undefined2 *puVar13;
  undefined2 *puVar14;
  undefined2 unaff_BP;
  int unaff_SI;
  undefined2 *puVar15;
  uint *puVar16;
  undefined2 uVar17;
  undefined2 unaff_DS;
  
  DAT_2000_b10e = 0x1000;
  DAT_2000_b10a = (undefined2 *)CONCAT22(0x7788,(undefined2 *)DAT_2000_b10a);
  FUN_1920_0000();
  DAT_2000_b10e = 0x1920;
  DAT_2000_b10a = (undefined2 *)CONCAT22(0x778d,(undefined2 *)DAT_2000_b10a);
  FUN_18ac_072d();
  DAT_2000_b10e = 0x18ac;
  DAT_2000_b10a = (undefined2 *)CONCAT22(0x7792,(undefined2 *)DAT_2000_b10a);
  FUN_184a_0000();
  DAT_2000_b10e = 0x184a;
  DAT_2000_b10a = (undefined2 *)CONCAT22(0x7797,(undefined2 *)DAT_2000_b10a);
  FUN_182d_0111();
  uVar17 = 0x1920;
  DAT_2000_b10a = (undefined2 *)0x182d77a2;
  DAT_2000_b10e = unaff_BP;
  FUN_1920_04df();
  puVar12 = (undefined1 *)0x3ff6;
  *(undefined2 *)0x6616 = 0;
  *(undefined2 *)0x6618 = 0;
  *(undefined2 *)0x661a = 0;
  *(undefined2 *)0x661c = 0;
  *(undefined2 *)0x2074 = 0;
  DAT_2000_b104._0_1_ = 0xbd;
  DAT_2000_b104._1_1_ = 0x77;
  FUN_1000_26e8();
  *(undefined1 *)0x2058 = 0x30;
LAB_1000_77c2:
  *(undefined2 *)(puVar12 + -2) = 0x77c5;
  FUN_1000_2361();
  if (*(char *)0x79b8 == '\0') {
    *(undefined2 *)(puVar12 + -2) = 0;
    *(undefined2 *)(puVar12 + -4) = 0x82c6;
    FUN_1000_00a3();
    uVar10 = DAT_2000_b10e;
    DAT_2000_b10a = (undefined2 *)CONCAT22(0x82ce,(undefined2 *)DAT_2000_b10a);
    DAT_2000_b10e = uVar17;
    cVar8 = FUN_1920_00e9();
    *(char *)(in_BX + unaff_SI) = *(char *)(in_BX + unaff_SI) + cVar8;
    DAT_2000_b10a = (undefined2 *)0x192082db;
    DAT_2000_b10e = uVar10;
    FUN_1920_04df();
    DAT_2000_b0fe = 0;
    DAT_2000_b0fc = 0;
    DAT_2000_b10a._0_2_ = (undefined2 *)*piRam0002b11c;
    DAT_2000_b10a._2_2_ = ((int *)piRam0002b11c)[1];
    DAT_2000_b106 = (uint *)CONCAT22(((undefined2 *)puRam0002b116)[1],*puRam0002b116);
    do {
      DAT_2000_b100._1_1_ = *(byte *)((int)(undefined2 *)DAT_2000_b10a + DAT_2000_b0fe);
      DAT_2000_b102._1_1_ = *(undefined1 *)((int)(undefined2 *)DAT_2000_b10a + DAT_2000_b0fe + 1);
      DAT_2000_b102._0_1_ = *(undefined1 *)((int)(undefined2 *)DAT_2000_b10a + DAT_2000_b0fe + 2);
      DAT_2000_b0fe = DAT_2000_b0fe + 3;
      DAT_2000_b104._1_1_ = (DAT_2000_b100._1_1_ >> 4) + 1;
      DAT_2000_b0f6 = DAT_2000_b104._1_1_ + DAT_2000_b0fc;
      if (DAT_2000_b0fc <= DAT_2000_b0f6) {
        DAT_2000_b0fa = DAT_2000_b0fc;
        while (*(undefined1 *)((int)DAT_2000_b106 + DAT_2000_b0fa) = DAT_2000_b102._1_1_,
              DAT_2000_b0fa != DAT_2000_b0f6) {
          DAT_2000_b0fa = DAT_2000_b0fa + 1;
        }
      }
      DAT_2000_b0fc = DAT_2000_b0fc + DAT_2000_b104._1_1_;
      if (DAT_2000_b0fc < uRam0002b114) {
        iVar11 = (uint)DAT_2000_b100._1_1_ % 0x10 + 1;
        DAT_2000_b104._1_1_ = (byte)iVar11;
        DAT_2000_b0f6 = iVar11 + DAT_2000_b0fc;
        uVar9 = DAT_2000_b0fc;
        if (DAT_2000_b0fc <= DAT_2000_b0f6) {
          while (DAT_2000_b0fa = uVar9,
                *(undefined1 *)((int)DAT_2000_b106 + DAT_2000_b0fa) = (undefined1)DAT_2000_b102,
                DAT_2000_b0fa != DAT_2000_b0f6) {
            uVar9 = DAT_2000_b0fa + 1;
          }
        }
        DAT_2000_b0fc = DAT_2000_b0fc + DAT_2000_b104._1_1_;
      }
    } while (DAT_2000_b0fc < uRam0002b114);
    return;
  }
  *(undefined2 *)(puVar12 + -2) = 0x77d2;
  FUN_1000_2efd();
  *(undefined2 *)0x78c2 = 0;
  *(undefined1 *)0x7f5e = 0;
LAB_1000_77dc:
  *(undefined2 *)(puVar12 + -2) = 0x77df;
  FUN_1000_2adc();
  *(undefined2 *)(puVar12 + -2) = uVar17;
  uVar17 = 0x18ac;
  *(undefined2 *)(puVar12 + -4) = 0x77e4;
  FUN_18ac_0022();
  *(undefined2 *)0xc1f8 = *(undefined2 *)0xc1ec;
  *(int *)0xc1f6 = *(int *)0xc1f8 - *(int *)0xc1ec;
  *(undefined1 *)0x79a3 = 1;
  while( true ) {
    *(undefined2 *)(puVar12 + -2) = 0x78;
    *(undefined2 *)(puVar12 + -4) = uVar17;
    *(undefined2 *)(puVar12 + -6) = 0x7806;
    uVar7 = FUN_1920_13a8();
    *(undefined1 *)0x79a8 = uVar7;
    *(undefined2 *)(puVar12 + -2) = 0x14;
    *(undefined2 *)(puVar12 + -4) = 0x1920;
    *(undefined2 *)(puVar12 + -6) = 0x7810;
    cVar8 = FUN_1920_13a8();
    *(char *)0x79ab = *(char *)0x79a8 + '\x14' + cVar8;
    *(undefined2 *)(puVar12 + -2) = 0x32;
    *(undefined2 *)(puVar12 + -4) = 0x1920;
    *(undefined2 *)(puVar12 + -6) = 0x7826;
    cVar8 = FUN_1920_13a8();
    *(char *)0x79a4 = cVar8 + 'P';
    if (*(char *)0x79a3 == '\x01') {
      *(undefined1 *)0x79a8 = 0;
      *(undefined1 *)0x79ab = 0xa0;
      *(undefined1 *)0x79a4 = 0x82;
    }
    if (*(byte *)0x79a4 < 0xa1) {
      *(uint *)0x2074 = (uint)*(byte *)0x79a4;
      while( true ) {
        DAT_2000_b10a = (undefined2 *)(ulong)CONCAT12(*(byte *)0x79ab,(undefined2 *)DAT_2000_b10a);
        puVar6 = DAT_2000_b10a;
        DAT_2000_b10a._2_2_ = (uint)*(byte *)0x79ab;
        bVar3 = *(byte *)0x79a8 <= DAT_2000_b10a._2_2_;
        DAT_2000_b10a = puVar6;
        if (bVar3) {
          *(uint *)0x2072 = (uint)*(byte *)0x79a8;
          while( true ) {
            *(undefined1 *)
             ((int)*(undefined4 *)0xc498 + *(int *)0x2074 * *(int *)0xc1ec + *(int *)0x2072) = 0xb0;
            if (*(int *)0x2072 == DAT_2000_b10a._2_2_) break;
            *(int *)0x2072 = *(int *)0x2072 + 1;
          }
        }
        if (*(int *)0x2074 == 0xa0) break;
        *(int *)0x2074 = *(int *)0x2074 + 1;
      }
    }
    if (*(char *)0x79a3 != '\x01') {
      *(undefined2 *)0x2074 = 1;
      while( true ) {
        *(uint *)(puVar12 + -2) = 0xa0 - (uint)*(byte *)0x79a4;
        *(undefined2 *)(puVar12 + -4) = 0x1920;
        *(undefined2 *)(puVar12 + -6) = 0x78ba;
        iVar11 = FUN_1920_13a8();
        *(uint *)(puVar12 + -2) = ((uint)*(byte *)0x79a4 + iVar11) * *(int *)0xc1ec;
        *(uint *)(puVar12 + -4) = (uint)*(byte *)0x79ab - (uint)*(byte *)0x79a8;
        *(undefined2 *)(puVar12 + -6) = 0x1920;
        *(undefined2 *)(puVar12 + -8) = 0x78dc;
        iVar11 = FUN_1920_13a8();
        *(undefined1 *)
         ((int)*(undefined4 *)0xc498 + (uint)*(byte *)0x79a8 + iVar11 + *(int *)(puVar12 + -2)) =
             0x16;
        if (*(int *)0x2074 == 0x14) break;
        *(int *)0x2074 = *(int *)0x2074 + 1;
      }
    }
    uVar17 = 0x1920;
    if (*(char *)0x79a3 == '\n') break;
    *(char *)0x79a3 = *(char *)0x79a3 + '\x01';
  }
LAB_1000_7903:
  *(int *)0x78c2 = *(int *)0x78c2 + 1;
  uVar7 = 0x1e;
  if (*(uint *)0x78c2 % 0x1e == 0) {
    pcVar4 = (code *)swi(0x21);
    (*pcVar4)();
    *(undefined1 *)0x79ab = (char)((uint)extraout_DX >> 8);
    *(undefined1 *)0x79ac = (char)extraout_DX;
    *(undefined1 *)0x79aa = uVar7;
    *(uint *)(puVar12 + -2) = (uint)*(byte *)0x79a9;
    in_BX = (uint)*(byte *)0x79ac;
    *(int *)0x2074 =
         (((uint)*(byte *)0x79aa - (uint)*(byte *)0x79a4) * 6000 +
          ((uint)*(byte *)0x79ab - (uint)*(byte *)0x79a8) * 100 + in_BX) - *(int *)(puVar12 + -2);
    uVar10 = 0;
    *(undefined2 *)(puVar12 + -2) = uVar17;
    *(undefined2 *)(puVar12 + -4) = 0x7970;
    uVar17 = FUN_1920_0f0f();
    *(undefined2 *)(puVar12 + -2) = uVar10;
    *(uint *)(puVar12 + -4) = in_BX;
    *(undefined2 *)(puVar12 + -6) = uVar17;
    *(undefined2 *)(puVar12 + -8) = 0x1920;
    *(undefined2 *)(puVar12 + -10) = 0x797d;
    FUN_1920_0f0f();
    *(undefined2 *)(puVar12 + -2) = 0x1920;
    *(undefined2 *)(puVar12 + -4) = 0x7985;
    FUN_1920_0f01();
    unaff_SI = 0;
    *(undefined2 *)(puVar12 + -2) = 0x1920;
    *(undefined2 *)(puVar12 + -4) = 0x7992;
    FUN_1920_0efb();
    *(undefined2 *)(puVar12 + -2) = 0x1920;
    uVar17 = 0x1920;
    *(undefined2 *)(puVar12 + -4) = 0x7997;
    uVar7 = FUN_1920_0f13();
    *(undefined1 *)0x79b5 = uVar7;
    if ((0x7d < *(uint *)0x2074) && (*(int *)0x78cc != 0)) {
      if (*(uint *)0x78ca < *(uint *)0x78cc) {
        *(int *)0x78cc = *(int *)0x78cc - *(int *)0x78ca;
      }
      else {
        *(undefined2 *)0x78cc = 0;
      }
    }
    if (*(uint *)0x2074 < 0x78) {
      *(int *)0x78cc = *(int *)0x78cc + *(int *)0x78ca;
    }
    *(undefined1 *)0x79a8 = *(undefined1 *)0x79ab;
    *(undefined1 *)0x79a9 = *(undefined1 *)0x79ac;
    *(undefined1 *)0x79a4 = *(undefined1 *)0x79aa;
  }
  *(undefined2 *)(puVar12 + -2) = *(undefined2 *)0x78cc;
  *(undefined2 *)(puVar12 + -4) = uVar17;
  *(undefined2 *)(puVar12 + -6) = 0x79e8;
  FUN_184a_029c();
  if ((*(char *)0x79b5 != *(char *)0x79b6) || (*(int *)0x2088 != *(int *)0x208a)) {
    *(undefined2 *)(puVar12 + -2) = 0xcb0f;
    *(undefined2 *)(puVar12 + -4) = 0x7a00;
    FUN_1000_3184();
  }
  while( true ) {
    *(undefined2 *)(puVar12 + -2) = 0x184a;
    uVar17 = 0x184a;
    *(undefined2 *)(puVar12 + -4) = 0x7a05;
    uVar9 = FUN_184a_02fd();
    if ((char)uVar9 == '\0') break;
    *(undefined2 *)(puVar12 + -2) = 0x184a;
    *(undefined2 *)(puVar12 + -4) = 0x7a0e;
    uVar7 = FUN_184a_030f();
    *(undefined1 *)0x2058 = uVar7;
  }
  if (*(char *)0x79e6 != '\0') {
    *(undefined2 *)(puVar12 + -2) = *(undefined2 *)0xc21e;
    *(undefined2 *)(puVar12 + -4) = *(undefined2 *)0xc220;
    *(undefined2 *)(puVar12 + -6) = 0x7a25;
    FUN_1000_3587();
    *(undefined2 *)(puVar12 + -2) = 0x184a;
    *(undefined2 *)(puVar12 + -4) = 0x7a2a;
    FUN_18ac_00f4();
    *(undefined2 *)0xc1f4 = *(undefined2 *)0x79f0;
    *(undefined2 *)(puVar12 + -2) = 0x18ac;
    uVar17 = 0x18ac;
    *(undefined2 *)(puVar12 + -4) = 0x7a35;
    uVar9 = FUN_18ac_03c8();
  }
  if (*(char *)0x79e7 != '\0') {
    *(undefined2 *)(puVar12 + -2) = *(undefined2 *)0xc226;
    *(undefined2 *)(puVar12 + -4) = *(undefined2 *)0xc228;
    *(undefined2 *)(puVar12 + -6) = 0x7a47;
    FUN_1000_3587();
    *(undefined2 *)(puVar12 + -2) = uVar17;
    *(undefined2 *)(puVar12 + -4) = 0x7a4c;
    FUN_18ac_00f4();
    *(undefined2 *)0xc1f4 = *(undefined2 *)0x79f2;
    *(undefined2 *)(puVar12 + -2) = 0x18ac;
    uVar17 = 0x18ac;
    *(undefined2 *)(puVar12 + -4) = 0x7a57;
    uVar9 = FUN_18ac_03c8();
  }
  *(undefined1 *)0x79e8 = 0;
  *(undefined1 *)0x79e9 = 0;
  *(undefined1 *)0x79ae = 0;
  *(undefined1 *)0x79af = 0;
  if (*(char *)0x79a6 != '\0') {
    *(undefined2 *)0x2082 = 1;
    while (uVar9 = (uint)*(byte *)0x79a6, *(uint *)0x2082 <= uVar9) {
      DAT_2000_b10a =
           (undefined2 *)CONCAT22(unaff_DS,(undefined2 *)(*(int *)0x2082 * 0x1e + 0x74a8));
      pcVar2 = (char *)(*(int *)0x2082 * 0x1e + 0x74c3);
      *pcVar2 = *pcVar2 + -1;
      uVar10 = (undefined2)((ulong)DAT_2000_b10a >> 0x10);
      puVar15 = (undefined2 *)DAT_2000_b10a;
      if ((((*(char *)((int)puVar15 + 0x1b) == '\0') && (*(char *)(puVar15 + 5) != '\0')) &&
          (*(char *)((int)puVar15 + 9) != '\0')) && (*(char *)(puVar15 + 4) == '\x01')) {
        *(undefined1 *)((int)puVar15 + 0x1b) = *(undefined1 *)(puVar15 + 0xe);
        *(undefined1 *)0x79a3 =
             *(undefined1 *)
              ((uint)*(byte *)((uint)*(byte *)((int)(undefined2 *)DAT_2000_b10a + 0xb) * 2 + 0x80) *
               2 + 0x58);
        *(undefined2 *)(puVar12 + -2) = *DAT_2000_b10a;
        *(undefined2 *)(puVar12 + -4) = ((undefined2 *)DAT_2000_b10a)[1];
        *(undefined2 *)(puVar12 + -6) = 0;
        *(undefined2 *)(puVar12 + -8) = 0;
        *(uint *)(puVar12 + -10) = (uint)*(byte *)0x79a3;
        *(uint *)(puVar12 + -0xc) = (uint)*(byte *)((int)(undefined2 *)DAT_2000_b10a + 0xb);
        *(undefined2 *)(puVar12 + -0xe) = 0;
        *(uint *)(puVar12 + -0x10) = (uint)*(byte *)((undefined2 *)DAT_2000_b10a + 0xd);
        *(undefined2 *)(puVar12 + -0x12) = 0x7b2b;
        FUN_1000_2f9f();
        if (*(int *)0x2072 == 1) {
          pcVar2 = (char *)((int)(undefined2 *)DAT_2000_b10a + 9);
          *pcVar2 = *pcVar2 + -1;
          *(char *)((undefined2 *)DAT_2000_b10a + 5) =
               *(char *)((undefined2 *)DAT_2000_b10a + 5) + -1;
          DAT_2000_b106 = (uint *)CONCAT22(unaff_DS,(uint *)((uint)*(byte *)0x208d * 0x26 + 0x1bae))
          ;
          *(undefined2 *)(puVar12 + -2) = ((undefined2 *)DAT_2000_b10a)[7];
          *(undefined2 *)(puVar12 + -4) = uVar17;
          *(undefined2 *)(puVar12 + -6) = 0x7b61;
          iVar11 = FUN_1920_13a8();
          ((uint *)DAT_2000_b106)[7] = iVar11 + ((undefined2 *)DAT_2000_b10a)[6];
          *(undefined2 *)(puVar12 + -2) = ((undefined2 *)DAT_2000_b10a)[9];
          *(undefined2 *)(puVar12 + -4) = 0x1920;
          *(undefined2 *)(puVar12 + -6) = 0x7b7b;
          iVar11 = FUN_1920_13a8();
          ((uint *)DAT_2000_b106)[8] = iVar11 + ((undefined2 *)DAT_2000_b10a)[8];
          *(undefined2 *)(puVar12 + -2) = ((undefined2 *)DAT_2000_b10a)[0xb];
          *(undefined2 *)(puVar12 + -4) = 0x1920;
          *(undefined2 *)(puVar12 + -6) = 0x7b95;
          iVar11 = FUN_1920_13a8();
          ((uint *)DAT_2000_b106)[9] = iVar11 + ((undefined2 *)DAT_2000_b10a)[10];
          *(uint *)(puVar12 + -2) = (uint)*(byte *)((int)(undefined2 *)DAT_2000_b10a + 0x19);
          *(undefined2 *)(puVar12 + -4) = 0x1920;
          uVar17 = 0x1920;
          *(undefined2 *)(puVar12 + -6) = 0x7bb2;
          cVar8 = FUN_1920_13a8();
          *(char *)((uint *)DAT_2000_b106 + 0x12) =
               *(char *)((undefined2 *)DAT_2000_b10a + 0xc) + cVar8;
          *(undefined1 *)((int)(uint *)DAT_2000_b106 + 3) =
               *(undefined1 *)((uint)*(byte *)((int)(undefined2 *)DAT_2000_b10a + 0xb) * 2 + 0x80);
          *(undefined1 *)((uint *)DAT_2000_b106 + 2) =
               *(undefined1 *)((uint)*(byte *)((int)(undefined2 *)DAT_2000_b10a + 0xb) * 2 + 0x81);
          puVar16 = (uint *)DAT_2000_b106;
          *(int *)(puVar12 + -2) = (int)((ulong)DAT_2000_b106 >> 0x10);
          *(uint **)(puVar12 + -4) = puVar16 + 0xb;
          *(uint *)(puVar12 + -6) = (uint)*(byte *)0x79a3;
          *(uint *)(puVar12 + -8) =
               (uint)*(byte *)((uint)*(byte *)((uint)*(byte *)((int)(undefined2 *)DAT_2000_b10a +
                                                              0xb) * 2 + 0x80) * 2 + 0x59);
          *(uint *)(puVar12 + -10) = (uint)*(byte *)((int)(undefined2 *)DAT_2000_b10a + 0x1d);
          *(undefined2 *)(puVar12 + -0xc) = 1;
          *(undefined2 *)(puVar12 + -0xe) = 0x7c2c;
          FUN_1000_06ab();
          *(undefined1 *)((int)(uint *)DAT_2000_b106 + 0x25) = *(undefined1 *)0x2082;
        }
      }
      *(int *)0x2082 = *(int *)0x2082 + 1;
    }
  }
  *(undefined2 *)0x2082 = 1;
  while( true ) {
    if ((*(char *)(*(int *)0x2082 + 0x1b75) != '\0') &&
       (*(char *)(*(int *)0x2082 + 0x79e5) == '\x01')) {
      iVar11 = *(int *)(*(int *)0x2082 * 2 + 0x79f2) + 0x79;
      *(int *)(puVar12 + -2) = iVar11;
      *(undefined2 *)(puVar12 + -4) = 0xa3;
      *(uint *)(puVar12 + -6) = CONCAT11((char)((uint)iVar11 >> 8),*(undefined1 *)0x2082);
      *(undefined2 *)(puVar12 + -8) = 0x7c77;
      uVar9 = FUN_1000_326e();
    }
    iVar11 = *(int *)0x2082 * 0x26;
    DAT_2000_b10a = (undefined2 *)CONCAT22(unaff_DS,(undefined2 *)(iVar11 + 0x1b62));
    if ((*(char *)(iVar11 + 0x1b77) == '\x02') &&
       (*(int *)(iVar11 + 0x1b72) = *(int *)(iVar11 + 0x1b72) + -1,
       ((undefined2 *)DAT_2000_b10a)[8] == 0)) {
      *(char *)(*(int *)0x2082 + 0x79e9) = *(char *)(*(int *)0x2082 + 0x79e9) + -1;
      if (*(char *)(*(int *)0x2082 + 0x79e9) == -1) {
        *(undefined1 *)(*(int *)0x2082 + 0x79e5) = 0;
        *(char *)0x79b8 = *(char *)0x79b8 + -1;
        *(int *)0x78cc = *(int *)0x78cc + 6;
        if (*(char *)0x79b8 == '\x01') {
          *(int *)(puVar12 + -2) = (*(int *)0x2082 + -1) * 0xa0;
          *(undefined2 *)(puVar12 + -4) = 0;
          *(undefined2 *)(puVar12 + -6) = 0xa0;
          *(undefined2 *)(puVar12 + -8) = 0xa0;
          *(undefined2 *)(puVar12 + -10) = 0;
          *(undefined2 *)(puVar12 + -0xc) = 0x7ce9;
          FUN_1000_07c5();
        }
        uVar9 = (uint)*(byte *)(*(int *)0x2082 * 0x26 + 0x1b63);
        *(undefined2 *)(uVar9 * 8 + -0x3de2) = 10000;
        if (*(char *)0x79b8 == '\0') {
          *(undefined2 *)(puVar12 + -2) = 1;
          *(undefined2 *)(puVar12 + -4) = 0x7d0b;
          FUN_1000_1b14();
          goto LAB_1000_77c2;
        }
      }
      else {
        iVar11 = *(int *)0x2082;
        *(undefined2 *)(puVar12 + -2) = unaff_DS;
        *(int *)(puVar12 + -4) = iVar11 * 0x26 + 0x1b62;
        *(uint *)(puVar12 + -6) = CONCAT11((char)(uVar9 >> 8),*(undefined1 *)0x2082);
        *(undefined2 *)(puVar12 + -8) = 0x7d23;
        FUN_1000_056b();
        *(undefined2 *)(puVar12 + -2) = *(undefined2 *)0x2082;
        *(undefined2 *)(puVar12 + -4) = 0x7d2a;
        FUN_1000_29db();
        if (*(byte *)(*(int *)0x2082 * 4 + 0x1b68) < 100) {
          *(undefined1 *)(*(int *)0x2082 * 4 + 0x1b68) = 100;
        }
        if (*(byte *)(*(int *)0x2082 * 4 + 0x1b69) < 10) {
          *(undefined1 *)(*(int *)0x2082 * 4 + 0x1b69) = 10;
        }
        if (*(byte *)(*(int *)0x2082 * 4 + 0x1b6a) < 2) {
          *(undefined1 *)(*(int *)0x2082 * 4 + 0x1b6a) = 2;
        }
        *(undefined1 *)(*(int *)0x2082 + 0x79e5) = 2;
        iVar11 = (*(int *)0x2082 + -1) * 8;
        DAT_2000_b106 = (uint *)CONCAT22(unaff_DS,(uint *)(iVar11 + -0x3de2));
        uVar9 = *(uint *)0xc3c0;
        *(uint *)(iVar11 + -0x3ddc) = uVar9;
        if (*(char *)0x79ca == '\0') {
          *(undefined1 *)((uint *)DAT_2000_b106 + 2) = 1;
          *(undefined1 *)((int)(uint *)DAT_2000_b106 + 5) = 1;
        }
      }
    }
    if (*(int *)0x2082 == 1) {
      *(undefined1 *)0x79a3 = *(undefined1 *)0x1b7b;
    }
    else {
      *(undefined1 *)0x79a3 = *(undefined1 *)0x1b80;
    }
    uVar9 = uVar9 & 0xff00;
    if (*(char *)0x79ca == '\0') {
      *(undefined1 *)0x79a3 = 0;
      *(undefined1 *)0x79b9 = 0xe5;
    }
    if (*(char *)(*(int *)0x2082 + 0x79e5) == '\x02') {
      iVar11 = (uint)*(byte *)(*(int *)0x2082 * 0x26 + 0x1b63) * 8;
      DAT_2000_b106 = (uint *)CONCAT22(unaff_DS,(uint *)(iVar11 + -0x3de2));
      *(int *)0x2074 =
           (*DAT_2000_b106 >> 3) + ((*(int *)(iVar11 + -0x3de0) + 7U >> 3) + 1) * *(int *)0xc204;
      uVar10 = (undefined2)((ulong)DAT_2000_b106 >> 0x10);
      if (((*(char *)((int)*(undefined4 *)0xc1e0 + *(int *)0x2074) != '\0') &&
          (uVar9 = *(uint *)0x2074, *(byte *)((int)*(undefined4 *)0xc1e0 + uVar9) < 0x4d)) ||
         ((uVar9 = *(int *)0x2074 + 1, *(char *)((int)*(undefined4 *)0xc1e0 + uVar9) != '\0' &&
          ((uVar9 = *(int *)0x2074 + 1, *(byte *)((int)*(undefined4 *)0xc1e0 + uVar9) < 0x4d &&
           (0x18 < ((uint *)DAT_2000_b106)[1])))))) {
        puVar1 = (uint *)DAT_2000_b106 + 1;
        *puVar1 = *puVar1 - 1;
      }
      if (*(char *)0x79a3 == '\x01') {
        uVar10 = (undefined2)((ulong)DAT_2000_b10a >> 0x10);
        if (*(int *)0x2082 == 1) {
          *(undefined1 *)((int)(undefined2 *)DAT_2000_b10a + 0x15) = 0;
        }
        else {
          *(undefined1 *)((int)(undefined2 *)DAT_2000_b10a + 0x15) = 1;
        }
        *(undefined1 *)(*(int *)0x2082 + 0x79e5) = 1;
        *(undefined1 *)0x1b7b = 0;
        *(undefined1 *)0x1b80 = 0;
        *(undefined1 *)(*(int *)0x2082 * 0x26 + 0x1b86) = 100;
      }
    }
    if (*(int *)0x2082 == 2) break;
    *(int *)0x2082 = *(int *)0x2082 + 1;
  }
  if (*(char *)0x79f9 != '\0') {
    *(undefined2 *)(puVar12 + -2) = 0x7ec5;
    FUN_1000_432a();
  }
  *(undefined2 *)0x2082 = 1;
  while (*(uint *)0x2082 <= (uint)*(byte *)0x208d) {
    iVar11 = *(int *)0x2082;
    *(undefined2 *)(puVar12 + -2) = unaff_DS;
    *(int *)(puVar12 + -4) = iVar11 * 0x26 + 0x1bae;
    puVar13 = (undefined2 *)(puVar12 + -6);
    puVar12 = puVar12 + -6;
    *puVar13 = 0x7ee4;
    FUN_1000_6053();
    *(int *)0x2082 = *(int *)0x2082 + 1;
  }
  if ((*(char *)0x79e6 == '\x01') || (*(char *)0x79e7 == '\x01')) {
    *(undefined1 *)0x79b9 = 0;
  }
  else {
    *(char *)0x79b9 = *(char *)0x79b9 + '\x01';
    if (*(char *)0x79b9 == -0x1a) {
      *(undefined2 *)0x2082 = 1;
      while( true ) {
        if (*(char *)(*(int *)0x2082 + 0x79e5) == '\x02') {
          *(undefined1 *)(*(int *)0x2082 + 0x79e5) = 1;
        }
        if (*(int *)0x2082 == 2) break;
        *(int *)0x2082 = *(int *)0x2082 + 1;
      }
      goto LAB_1000_77dc;
    }
  }
  *(undefined2 *)0x2082 = 1;
  while( true ) {
    if (*(char *)(*(int *)0x2082 + 0x79e5) == '\x01') {
      iVar11 = *(int *)0x2082;
      *(undefined2 *)(puVar12 + -2) = unaff_DS;
      *(int *)(puVar12 + -4) = iVar11 * 0x26 + 0x1b62;
      puVar14 = (undefined2 *)(puVar12 + -6);
      puVar12 = puVar12 + -6;
      *puVar14 = 0x7f5c;
      FUN_1000_6053();
      if (*(char *)(*(int *)0x2082 * 0x26 + 0x1b77) != '\x02') {
        pcVar2 = (char *)(*(int *)0x2082 * 0x26 + 0x1b86);
        *pcVar2 = *pcVar2 - *(char *)(*(int *)0x2082 + 0x79e7);
      }
      if (*(char *)(*(int *)0x2082 + 0x79e7) != '\0') {
        *(undefined2 *)0x2074 = 0x2d;
        *(undefined1 *)0x799f = 4;
        *(undefined2 *)(puVar12 + -2) = 0x7f92;
        FUN_1000_165a();
      }
      *(uint *)0x2074 = (uint)*(byte *)(*(int *)0x2082 * 0x26 + 0x1b86);
      if (200 < *(uint *)0x2074) {
        *(uint *)(puVar12 + -2) = (uint)*(byte *)0x2082;
        *(undefined2 *)(puVar12 + -4) = 0x7faf;
        FUN_1000_30a3();
      }
      if ((uint)*(byte *)(*(int *)0x2082 + 0x79eb) != *(uint *)0x2074) {
        *(undefined1 *)(*(int *)0x2082 + 0x79eb) = *(undefined1 *)0x2074;
        *(int *)0x2072 = *(int *)(*(int *)0x2082 * 2 + 0x79f2) + 1;
        *(undefined2 *)(puVar12 + -2) = 0x7fdb;
        FUN_1000_568a();
      }
      if (*(int *)0x2082 == 1) {
        *(undefined2 *)0x2072 = 0;
        *(undefined2 *)0x78b6 = 0x785a;
        *(undefined2 *)0x78b8 = unaff_DS;
      }
      else {
        *(undefined2 *)0x2072 = *(undefined2 *)0x2084;
        *(undefined2 *)0x78b6 = 0x7888;
        *(undefined2 *)0x78b8 = unaff_DS;
      }
      if (*(byte *)((int)*(undefined4 *)0x78b6 + 0x2c) < 2) {
        if (*(char *)((int)*(undefined4 *)0x78b6 + 0x2c) == '\0') {
          uVar5 = *(undefined4 *)0x78b6;
          *(int *)(puVar12 + -2) = (int)((ulong)uVar5 >> 0x10);
          *(int *)(puVar12 + -4) = (int)uVar5;
          *(undefined2 *)(puVar12 + -6) = 0x8026;
          FUN_1000_11b0();
        }
        *(int *)0x2074 = *(int *)(*(int *)0x2082 * 2 + 0x79f2) + -0x2400;
        *(undefined2 *)(puVar12 + -2) = *(undefined2 *)0x2072;
        *(undefined2 *)(puVar12 + -4) = 0x803d;
        FUN_1000_1287();
        *(undefined2 *)(puVar12 + -2) = *(undefined2 *)0x2072;
        *(undefined2 *)(puVar12 + -4) = 0x8044;
        FUN_1000_1239();
      }
    }
    if (*(int *)0x2082 == 2) break;
    *(int *)0x2082 = *(int *)0x2082 + 1;
  }
  if ((*(int *)0x2076 != 0) || (199 < *(uint *)0x207e)) {
    *(undefined2 *)(puVar12 + -2) = 0x8060;
    FUN_1000_45fa();
  }
  if (*(int *)0x2080 != 0) {
    *(undefined2 *)(puVar12 + -2) = 0x806a;
    FUN_1000_5102();
  }
  if (*(int *)0x209c != 0) {
    *(int *)(puVar12 + -2) = *(int *)0x209a << 1;
    *(undefined2 *)(puVar12 + -4) = uVar17;
    uVar17 = 0x1920;
    *(undefined2 *)(puVar12 + -6) = 0x807c;
    uVar10 = FUN_1920_13a8();
    *(undefined2 *)0x2098 = uVar10;
    *(int *)0x209a = *(int *)0x209a + -1;
    if (*(int *)0x209a == 0) {
      *(undefined2 *)0x209c = 0;
      *(undefined2 *)0x2098 = 0;
    }
  }
  if (*(char *)0x79cb != '\0') {
    DAT_2000_b10a = (undefined2 *)(ulong)CONCAT12(*(char *)0x79cb,(undefined2 *)DAT_2000_b10a);
    if (*(char *)0x79cb != '\0') {
      *(undefined2 *)0x2082 = 1;
      while( true ) {
        *(undefined1 *)0x208c = 0;
        *(undefined1 *)0x79a3 = 1;
        while( true ) {
          if (*(char *)(*(int *)0x2082 * 3 + (uint)*(byte *)0x79a3 + 0x79ce) !=
              *(char *)(*(int *)0x2082 * 3 + (uint)*(byte *)0x79a3 + 0x79d8)) {
            *(undefined1 *)0x208c = 1;
            if (*(byte *)(*(int *)0x2082 * 3 + (uint)*(byte *)0x79a3 + 0x79d8) <
                *(byte *)(*(int *)0x2082 * 3 + (uint)*(byte *)0x79a3 + 0x79ce)) {
              pcVar2 = (char *)(*(int *)0x2082 * 3 + (uint)*(byte *)0x79a3 + 0x79ce);
              *pcVar2 = *pcVar2 + -2;
            }
            else {
              pcVar2 = (char *)(*(int *)0x2082 * 3 + (uint)*(byte *)0x79a3 + 0x79ce);
              *pcVar2 = *pcVar2 + '\x02';
            }
          }
          if (*(char *)0x79a3 == '\x03') break;
          *(char *)0x79a3 = *(char *)0x79a3 + '\x01';
        }
        if ((*(char *)0x208c == '\0') && ((uint)*(byte *)0x79cb == *(uint *)0x2082)) {
          *(char *)0x79cb = *(char *)0x79cb + -1;
        }
        *(uint *)(puVar12 + -2) = (uint)*(byte *)(*(int *)0x2082 + 0x79cd);
        *(uint *)(puVar12 + -4) = (uint)*(byte *)(*(int *)0x2082 * 3 + 0x79cf);
        *(uint *)(puVar12 + -6) = (uint)*(byte *)(*(int *)0x2082 * 3 + 0x79d0);
        unaff_SI = *(int *)0x2082;
        *(uint *)(puVar12 + -8) = (uint)*(byte *)(unaff_SI * 3 + 0x79d1);
        *(undefined2 *)(puVar12 + -10) = uVar17;
        uVar17 = 0x18ac;
        *(undefined2 *)(puVar12 + -0xc) = 0x81b4;
        FUN_18ac_0000();
        if (*(int *)0x2082 == DAT_2000_b10a._2_2_) break;
        *(int *)0x2082 = *(int *)0x2082 + 1;
      }
    }
  }
  if (*(char *)0x79b2 != '\0') {
    *(char *)0x79b2 = *(char *)0x79b2 + -1;
    *(undefined2 *)(puVar12 + -2) = 0x81cd;
    FUN_1000_3f27();
  }
  if (*(uint *)0x78c2 % 5 == 0) {
    *(undefined2 *)(puVar12 + -2) = 0x81df;
    FUN_1000_079d();
    *(char *)0x79ad = *(char *)0x79ad + '\a';
    if (0x3f < *(byte *)0x79ad) {
      *(undefined1 *)0x79ad = 0x14;
    }
  }
  if (*(char *)0x2058 == 's') {
    *(bool *)0xc49c = *(char *)0xc49c == '\0';
    *(undefined1 *)0x2058 = 0x39;
  }
  if (((*(char *)0x2058 == 'e') && (0x15 < *(uint *)0xc1ea)) && (*(char *)0x79b8 == '\x01')) {
    *(undefined2 *)(puVar12 + -2) = 0;
    *(undefined2 *)(puVar12 + -4) = 0;
    *(undefined2 *)(puVar12 + -6) = 0x140;
    *(undefined2 *)(puVar12 + -8) = 0xa0;
    *(undefined2 *)(puVar12 + -10) = 0;
    *(undefined2 *)(puVar12 + -0xc) = 0x8233;
    FUN_1000_07c5();
    *(int *)(puVar12 + -2) = *(int *)0xc1ea + -1;
    *(undefined2 *)(puVar12 + -4) = 0x823b;
    FUN_1000_0838();
    *(undefined1 *)0x2058 = 0x39;
    *(undefined2 *)0x79ee = *(undefined2 *)0xc1ea;
    *(undefined2 *)(puVar12 + -2) = 0x8249;
    FUN_1000_2959();
  }
  if (((*(char *)0x2058 == 'r') && (*(uint *)0xc1ea < 0x28)) && (*(char *)0x79b8 == '\x01')) {
    *(undefined2 *)(puVar12 + -2) = 0;
    *(undefined2 *)(puVar12 + -4) = 0;
    *(undefined2 *)(puVar12 + -6) = 0x140;
    *(undefined2 *)(puVar12 + -8) = 0xa0;
    *(undefined2 *)(puVar12 + -10) = 0;
    *(undefined2 *)(puVar12 + -0xc) = 0x826d;
    FUN_1000_07c5();
    *(int *)(puVar12 + -2) = *(int *)0xc1ea + 1;
    *(undefined2 *)(puVar12 + -4) = 0x8275;
    FUN_1000_0838();
    *(undefined1 *)0x2058 = 0x39;
    *(undefined2 *)0x79ee = *(undefined2 *)0xc1ea;
    *(undefined2 *)(puVar12 + -2) = 0x8283;
    FUN_1000_2959();
  }
  if (((*(char *)0x79c5 != '\0') && (*(char *)0x79c6 != '\0')) && (*(int *)0x2080 == 0))
  goto code_r0x00018298;
  if (*(char *)0x2058 == '\x1b') {
    *(undefined2 *)(puVar12 + -2) = 1;
    *(undefined2 *)(puVar12 + -4) = 0x82be;
    FUN_1000_1b14();
    goto LAB_1000_77c2;
  }
  goto LAB_1000_7903;
code_r0x00018298:
  *(undefined2 *)(puVar12 + -2) = 0x829b;
  FUN_1000_1d61();
  if (7 < *(byte *)0x79b7) goto LAB_1000_82a7;
  goto LAB_1000_77dc;
LAB_1000_82a7:
  *(undefined2 *)(puVar12 + -2) = 2;
  *(undefined2 *)(puVar12 + -4) = 0x82ac;
  FUN_1000_1b14();
  goto LAB_1000_77c2;
}



// ================================================
// Function: FUN_182d_0000 at 182d:0000
// ================================================

void __stdcall16far FUN_182d_0000(uint param_1,int *param_2,undefined2 param_3,int *param_4)

{
  byte bVar1;
  undefined1 uVar2;
  undefined1 uVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  byte bVar8;
  uint uVar9;
  int iVar10;
  uint local_16;
  uint local_14;
  int local_12;
  
  FUN_1920_04df();
  local_12 = 0;
  local_14 = 0;
  iVar4 = *param_4;
  iVar5 = ((int *)param_4)[1];
  iVar6 = *param_2;
  iVar7 = ((int *)param_2)[1];
  do {
    bVar1 = *(byte *)(iVar4 + local_12);
    uVar2 = *(undefined1 *)(iVar4 + local_12 + 1);
    uVar3 = *(undefined1 *)(iVar4 + local_12 + 2);
    local_12 = local_12 + 3;
    bVar8 = (bVar1 >> 4) + 1;
    uVar9 = bVar8 + local_14;
    if (local_14 <= uVar9) {
      for (local_16 = local_14; *(undefined1 *)(iVar6 + local_16) = uVar2, local_16 != uVar9;
          local_16 = local_16 + 1) {
      }
    }
    local_14 = local_14 + bVar8;
    if (local_14 < param_1) {
      iVar10 = (uint)bVar1 % 0x10 + 1;
      uVar9 = iVar10 + local_14;
      local_16 = local_14;
      if (local_14 <= uVar9) {
        for (; *(undefined1 *)(iVar6 + local_16) = uVar3, local_16 != uVar9; local_16 = local_16 + 1
            ) {
        }
      }
      local_14 = local_14 + iVar10;
    }
  } while (local_14 < param_1);
  return;
}



// ================================================
// Function: FUN_182d_0111 at 182d:0111
// ================================================

void __cdecl16far FUN_182d_0111(void)

{
  FUN_1920_04df();
  return;
}



// ================================================
// Function: FUN_183f_0000 at 183f:0000
// ================================================

void __stdcall16far FUN_183f_0000(void)

{
  byte *pbVar1;
  code *pcVar2;
  byte bVar3;
  byte *pbVar5;
  byte *pbVar6;
  undefined2 unaff_SS;
  byte *in_stack_0000000a;
  int iVar4;
  
  pbVar5 = (byte *)in_stack_0000000a;
  pbVar6 = &stack0xffae;
  bVar3 = *in_stack_0000000a;
  if (0x4e < bVar3) {
    bVar3 = 0x4f;
  }
  for (iVar4 = (int)(char)bVar3; pbVar5 = pbVar5 + 1, iVar4 != 0; iVar4 = iVar4 + -1) {
    pbVar1 = pbVar6;
    pbVar6 = pbVar6 + 1;
    *pbVar1 = *pbVar5;
  }
  *pbVar6 = 0;
  pcVar2 = (code *)swi(0x21);
  (*pcVar2)();
  pcVar2 = (code *)swi(0x21);
  (*pcVar2)();
  FUN_183f_0058();
  return;
}



// ================================================
// Function: FUN_183f_003e at 183f:003e
// ================================================

void __stdcall16far FUN_183f_003e(void)

{
  code *pcVar1;
  
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  FUN_183f_0058();
  return;
}



// ================================================
// Function: FUN_183f_0058 at 183f:0058
// ================================================

void __cdecl16near FUN_183f_0058(void)

{
  byte *pbVar1;
  undefined2 in_AX;
  int iVar2;
  int iVar3;
  byte *pbVar4;
  int unaff_DI;
  byte *pbVar5;
  byte *pbVar6;
  undefined2 unaff_ES;
  undefined2 unaff_DS;
  bool in_CF;
  
  if (!in_CF) {
    pbVar5 = (byte *)(unaff_DI + 0x1e);
    iVar2 = 0x100;
    do {
      if (iVar2 == 0) break;
      iVar2 = iVar2 + -1;
      pbVar1 = pbVar5;
      pbVar5 = pbVar5 + 1;
    } while (*pbVar1 != 0);
    iVar3 = CONCAT11((char)((uint)iVar2 >> 8),~(byte)iVar2);
    pbVar6 = pbVar5;
    while( true ) {
      pbVar6 = pbVar6 + -1;
      pbVar4 = pbVar5 + -2;
      if (iVar3 == 0) break;
      iVar3 = iVar3 + -1;
      pbVar5 = pbVar5 + -1;
      *pbVar6 = *pbVar4;
    }
    *pbVar6 = ~(byte)iVar2;
    in_AX = 0;
  }
  *(undefined2 *)0x7f5a = in_AX;
  return;
}



// ================================================
// Function: FUN_183f_007b at 183f:007b
// ================================================

void __stdcall16far FUN_183f_007b(undefined2 *param_1)

{
  code *pcVar1;
  undefined2 in_BX;
  undefined2 unaff_ES;
  
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  *param_1 = in_BX;
  ((undefined2 *)param_1)[1] = unaff_ES;
  return;
}



// ================================================
// Function: FUN_183f_0093 at 183f:0093
// ================================================

void __stdcall16far FUN_183f_0093(void)

{
  code *pcVar1;
  
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  return;
}



// ================================================
// Function: FUN_184a_0000 at 184a:0000
// ================================================

void __cdecl16far FUN_184a_0000(void)

{
  undefined2 unaff_DS;
  
  FUN_184a_002e();
  FUN_184a_0331(0xc49e,unaff_DS);
  FUN_1920_0567(0x184a,0xc49e);
  FUN_184a_0331(0xc59e,unaff_DS);
  FUN_1920_056c(0x1920,0xc59e);
  return;
}



// ================================================
// Function: FUN_184a_002e at 184a:002e
// ================================================

/* WARNING: Removing unreachable block (ram,0x00018514) */

void __cdecl16near FUN_184a_002e(void)

{
  byte bVar1;
  byte extraout_AH;
  undefined2 unaff_DS;
  
  bVar1 = FUN_184a_060b();
  if ((bVar1 != 7) && (3 < bVar1)) {
    FUN_184a_0097();
  }
  FUN_184a_00e5();
  FUN_184a_060b();
  *(byte *)0x7f6c = extraout_AH & 0x7f;
  *(byte *)0x7f62 = extraout_AH & 0x7f;
  *(undefined1 *)0x7f5d = 0;
  *(undefined1 *)0x7f6d = 0;
  *(undefined1 *)0x7f6e = 0;
  *(undefined1 *)0x7f5c = 1;
  do {
  } while( true );
}



// ================================================
// Function: FUN_184a_0097 at 184a:0097
// ================================================

void __cdecl16near FUN_184a_0097(void)

{
  undefined2 in_AX;
  char extraout_DL;
  
  DAT_0000_0487 = DAT_0000_0487 & 0xfe;
  if (((byte)in_AX != 7) && (3 < (byte)in_AX)) {
    in_AX = CONCAT11((char)((uint)in_AX >> 8),3);
  }
  FUN_184a_060b();
  if ((char)((uint)in_AX >> 8) != '\0') {
    FUN_184a_060b();
    FUN_184a_060b();
    if (extraout_DL == '*') {
      DAT_0000_0487 = DAT_0000_0487 | 1;
      FUN_184a_060b();
      FUN_184a_060b();
    }
  }
  return;
}



// ================================================
// Function: FUN_184a_00e5 at 184a:00e5
// ================================================

void __cdecl16near FUN_184a_00e5(void)

{
  uint uVar1;
  uint uVar2;
  undefined1 uVar3;
  byte extraout_DL;
  byte bVar4;
  undefined2 uVar5;
  undefined2 unaff_DS;
  
  uVar1 = FUN_184a_060b();
  FUN_184a_060b();
  uVar3 = 0;
  bVar4 = extraout_DL;
  if ((extraout_DL == 0) && (bVar4 = 0x18, (byte)uVar1 < 4)) {
    uVar3 = 1;
  }
  uVar5 = CONCAT11(bVar4,(char)(uVar1 >> 8) + -1);
  uVar2 = uVar1 & 0xff;
  if (0x18 < bVar4) {
    uVar2 = CONCAT11(1,(byte)uVar1);
  }
  *(uint *)0x7f60 = uVar2;
  *(undefined2 *)0x7f6a = uVar5;
  *(undefined1 *)0x7f5f = uVar3;
  *(undefined1 *)0x7f5e = 1;
  *(undefined2 *)0x7f64 = 0;
  *(undefined2 *)0x7f66 = uVar5;
  return;
}



// ================================================
// Function: FUN_184a_0143 at 184a:0143
// ================================================

void __cdecl16near FUN_184a_0143(void)

{
  code *pcVar1;
  undefined2 unaff_DS;
  undefined1 uVar2;
  
  if (*(char *)0x7f6e == '\0') {
    return;
  }
  *(undefined1 *)0x7f6e = 0;
  uVar2 = 0;
  while( true ) {
    pcVar1 = (code *)swi(0x16);
    (*pcVar1)();
    if ((bool)uVar2) break;
    pcVar1 = (code *)swi(0x16);
    (*pcVar1)();
  }
  FUN_184a_047e();
  FUN_184a_047e();
  FUN_184a_0477();
  pcVar1 = (code *)swi(0x23);
  (*pcVar1)();
  FUN_184a_0097();
  FUN_184a_00e5();
  *(undefined1 *)0x7f62 = *(undefined1 *)0x7f6c;
  return;
}



// ================================================
// Function: FUN_184a_029c at 184a:029c
// ================================================

void __stdcall16far FUN_184a_029c(int param_1)

{
  int extraout_DX;
  
  if (param_1 != 0) {
    do {
      FUN_184a_02bd();
    } while (extraout_DX != 1);
  }
  return;
}



// ================================================
// Function: FUN_184a_02bd at 184a:02bd
// ================================================

void __cdecl16near FUN_184a_02bd(void)

{
  char in_AL;
  int in_CX;
  int iVar1;
  char *unaff_DI;
  undefined2 unaff_ES;
  
  do {
    iVar1 = 4;
    do {
      iVar1 = iVar1 + -1;
    } while (iVar1 != 0);
    in_CX = in_CX + -1;
  } while (in_CX != 0 && in_AL == *unaff_DI);
  return;
}



// ================================================
// Function: FUN_184a_02f6 at 184a:02f6
// ================================================

byte __cdecl16far FUN_184a_02f6(void)

{
  byte bVar1;
  
  bVar1 = in(0x61);
  out(0x61,bVar1 & 0xfc);
  return bVar1 & 0xfc;
}



// ================================================
// Function: FUN_184a_02fd at 184a:02fd
// ================================================

undefined1 __cdecl16far FUN_184a_02fd(void)

{
  code *pcVar1;
  undefined2 unaff_DS;
  bool bVar2;
  
  bVar2 = *(char *)0x7f6d == '\0';
  if (bVar2) {
    pcVar1 = (code *)swi(0x16);
    (*pcVar1)();
    if (bVar2) {
      return 0;
    }
  }
  return 1;
}



// ================================================
// Function: FUN_184a_030f at 184a:030f
// ================================================

void __cdecl16far FUN_184a_030f(void)

{
  code *pcVar1;
  char cVar2;
  undefined1 extraout_AH;
  undefined2 unaff_DS;
  
  cVar2 = *(char *)0x7f6d;
  *(undefined1 *)0x7f6d = 0;
  if (cVar2 == '\0') {
    pcVar1 = (code *)swi(0x16);
    cVar2 = (*pcVar1)();
    if (cVar2 == '\0') {
      *(undefined1 *)0x7f6d = extraout_AH;
    }
  }
  FUN_184a_0143();
  return;
}



// ================================================
// Function: FUN_184a_0331 at 184a:0331
// ================================================

void __stdcall16far FUN_184a_0331(undefined4 param_1)

{
  int iVar1;
  undefined2 uVar2;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  *(undefined2 *)(iVar1 + 2) = 0xd7b0;
  *(undefined2 *)(iVar1 + 4) = 0x80;
  *(int *)(iVar1 + 0xc) = iVar1 + 0x80;
  *(undefined2 *)(iVar1 + 0xe) = uVar2;
  *(undefined2 *)(iVar1 + 0x10) = 0x35c;
  *(undefined2 *)(iVar1 + 0x12) = 0x184a;
  *(undefined1 *)(iVar1 + 0x30) = 0;
  return;
}



// ================================================
// Function: FUN_184a_0394 at 184a:0394
// ================================================

void __stdcall16far FUN_184a_0394(undefined4 param_1)

{
  byte bVar1;
  int iVar2;
  uint uVar3;
  uint extraout_DX;
  uint extraout_DX_00;
  uint extraout_DX_01;
  uint uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  undefined2 uVar8;
  undefined2 uVar9;
  undefined2 unaff_DS;
  
  uVar8 = (undefined2)((ulong)param_1 >> 0x10);
  iVar6 = (int)param_1;
  uVar3 = *(int *)(iVar6 + 4) - 2;
  uVar9 = (undefined2)((ulong)*(undefined4 *)(iVar6 + 0xc) >> 0x10);
  iVar7 = (int)*(undefined4 *)(iVar6 + 0xc);
  uVar4 = 0;
  uVar5 = *(uint *)(iVar6 + 8);
LAB_184a_03aa:
  do {
    *(undefined1 *)0x7f6d = 0;
    bVar1 = FUN_184a_030f();
    iVar2 = 1;
    if ((bVar1 == 8) || (bVar1 == 0x13)) {
LAB_184a_03ee:
      do {
        if (uVar4 == 0) break;
        FUN_184a_047e();
        FUN_184a_047e();
        FUN_184a_047e();
        uVar4 = uVar4 - 1;
        iVar2 = iVar2 + -1;
        uVar3 = extraout_DX_00;
      } while (iVar2 != 0);
      goto LAB_184a_03aa;
    }
    if (bVar1 == 4) {
LAB_184a_0406:
      do {
        if ((uVar4 == uVar5) || (*(byte *)(uVar4 + iVar7) < 0x20)) break;
        FUN_184a_047e();
        uVar4 = uVar4 + 1;
        iVar2 = iVar2 + -1;
        uVar3 = extraout_DX_01;
      } while (iVar2 != 0);
    }
    else {
      iVar2 = 0;
      if ((bVar1 == 0x1b) || (bVar1 == 1)) goto LAB_184a_03ee;
      if (bVar1 == 6) goto LAB_184a_0406;
      if (bVar1 == 0x1a) {
        if (*(char *)0x7f5d != '\0') {
          *(undefined1 *)(uVar4 + iVar7) = 0x1a;
          iVar7 = uVar4 + 1;
LAB_184a_0430:
          *(undefined2 *)(iVar6 + 8) = 0;
          *(int *)(iVar6 + 10) = iVar7;
          return;
        }
      }
      else {
        if (bVar1 == 0xd) {
          FUN_184a_0477();
          *(undefined2 *)(uVar4 + iVar7) = 0xa0d;
          iVar7 = uVar4 + 2;
          goto LAB_184a_0430;
        }
        if ((0x1f < bVar1) && (uVar4 != uVar3)) {
          *(byte *)(uVar4 + iVar7) = bVar1;
          uVar4 = uVar4 + 1;
          FUN_184a_047e();
          uVar3 = extraout_DX;
          if (uVar5 < uVar4) {
            uVar5 = uVar4;
          }
        }
      }
    }
  } while( true );
}



// ================================================
// Function: FUN_184a_0477 at 184a:0477
// ================================================

/* WARNING: Removing unreachable block (ram,0x00018966) */
/* WARNING: Removing unreachable block (ram,0x0001895c) */
/* WARNING: Removing unreachable block (ram,0x00018962) */
/* WARNING: Removing unreachable block (ram,0x00018937) */
/* WARNING: Removing unreachable block (ram,0x0001894f) */
/* WARNING: Removing unreachable block (ram,0x00018955) */

void FUN_184a_0477(void)

{
  FUN_184a_047e();
  thunk_FUN_184a_060b();
  FUN_184a_04d7();
  thunk_FUN_184a_060b();
  return;
}



// ================================================
// Function: FUN_184a_047e at 184a:047e
// ================================================

void __cdecl16near FUN_184a_047e(void)

{
  char in_AL;
  char extraout_DL;
  undefined2 unaff_DS;
  
  thunk_FUN_184a_060b();
  if (in_AL == '\a') {
    FUN_184a_060b();
  }
  else if (((in_AL != '\b') && (in_AL != '\r')) &&
          ((in_AL == '\n' || (FUN_184a_060b(), *(byte *)0x7f66 < (byte)(extraout_DL + 1U))))) {
    FUN_184a_04d7();
  }
  thunk_FUN_184a_060b();
  return;
}



// ================================================
// Function: FUN_184a_04d7 at 184a:04d7
// ================================================

void __cdecl16near FUN_184a_04d7(void)

{
  undefined2 in_DX;
  undefined2 unaff_DS;
  
  if (*(byte *)0x7f67 < (byte)((char)((uint)in_DX >> 8) + 1U)) {
    FUN_184a_060b();
  }
  return;
}



// ================================================
// Function: thunk_FUN_184a_060b at 184a:04f8
// ================================================

void __cdecl16near thunk_FUN_184a_060b(void)

{
  FUN_184a_060b();
  return;
}



// ================================================
// Function: thunk_FUN_184a_060b at 184a:04ff
// ================================================

void __cdecl16near thunk_FUN_184a_060b(void)

{
  FUN_184a_060b();
  return;
}



// ================================================
// Function: FUN_184a_060b at 184a:060b
// ================================================

void __cdecl16near FUN_184a_060b(void)

{
  code *pcVar1;
  
  pcVar1 = (code *)swi(0x10);
  (*pcVar1)();
  return;
}



// ================================================
// Function: FUN_18ac_0000 at 18ac:0000
// ================================================

undefined4 __stdcall16far
FUN_18ac_0000(undefined1 param_1,undefined1 param_2,undefined1 param_3,undefined1 param_4)

{
  undefined1 extraout_AH;
  
  FUN_1920_04df();
  out(0x3c8,param_4);
  out(0x3c9,param_3);
  out(0x3c9,param_2);
  out(0x3c9,param_1);
  return CONCAT22(0x3c9,CONCAT11(extraout_AH,param_1));
}



// ================================================
// Function: FUN_18ac_0022 at 18ac:0022
// ================================================

void __cdecl16far FUN_18ac_0022(void)

{
  undefined2 unaff_DS;
  byte local_3;
  
  FUN_1920_04df();
  local_3 = 0xb0;
  while( true ) {
    FUN_18ac_0000((int)((local_3 - 0xb0) * -0xc) / 0x26 + 0xe,(int)((local_3 - 0xb0) * 0x17) / 0x26,
                  (int)((local_3 - 0xb0) * 0x2b) / 0x26,local_3);
    if (local_3 == 0xd6) break;
    local_3 = local_3 + 1;
  }
  *(undefined2 *)0xc1e8 = 0;
  while (*(char *)((int)*(undefined4 *)0xc498 + *(int *)0xc1e8) =
              (char)(*(uint *)0xc1e8 / (uint)(*(int *)0xc1ec << 2)) + -0x50,
        *(int *)0xc1e8 != -0x15a1) {
    *(int *)0xc1e8 = *(int *)0xc1e8 + 1;
  }
  return;
}



// ================================================
// Function: FUN_18ac_00f4 at 18ac:00f4
// ================================================

void __cdecl16far FUN_18ac_00f4(void)

{
  byte *pbVar1;
  char *pcVar2;
  undefined2 *puVar3;
  uint *puVar4;
  undefined2 *puVar5;
  uint *puVar6;
  undefined2 uVar7;
  undefined2 uVar8;
  undefined4 uVar9;
  uint *puVar10;
  uint uVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  int iVar15;
  int iVar16;
  uint uVar17;
  byte *pbVar18;
  byte *pbVar19;
  char *pcVar20;
  undefined2 *puVar21;
  undefined2 *puVar22;
  uint *puVar23;
  char *pcVar24;
  undefined2 unaff_DS;
  
  if (*(char *)0xc49c == '\x01') {
    uVar9 = *(undefined4 *)0xc498;
    puVar21 = (undefined2 *)
              ((int)uVar9 + (uint)(*(int *)0xc216 + *(int *)0xc20a) / 4 +
              ((uint)(*(int *)0xc218 + *(int *)0xc20c) >> 3) * *(int *)0xc1f8);
    uVar7 = *(undefined2 *)0xc212;
    puVar22 = (undefined2 *)(*(int *)0xc214 + *(int *)0xc20a + *(int *)0xc20c * *(int *)0xc1ec);
    uVar17 = *(uint *)0xc1ec;
    iVar12 = *(int *)0xc1ee;
    iVar13 = *(int *)0xc1f6;
    uVar11 = uVar17 >> 1;
    do {
      for (; uVar11 != 0; uVar11 = uVar11 - 1) {
        puVar5 = puVar22;
        puVar22 = puVar22 + 1;
        puVar3 = puVar21;
        puVar21 = puVar21 + 1;
        *puVar5 = *puVar3;
      }
      puVar21 = (undefined2 *)((int)puVar21 + iVar13);
      iVar12 = iVar12 + -1;
      uVar11 = uVar17 >> 1;
    } while (iVar12 != 0);
  }
  puVar23 = (uint *)*(undefined2 *)0xc214;
  uVar7 = *(undefined2 *)0xc212;
  pbVar18 = (byte *)*(undefined2 *)0xc1f0;
  iVar12 = 0x14;
  if (*(char *)0xc49c == '\x01') {
    do {
      *(undefined2 *)0xc210 = *(undefined2 *)0xc208;
      iVar13 = 8;
      do {
        iVar15 = *(int *)0xc1ea;
        pbVar19 = pbVar18;
        do {
          pbVar1 = pbVar19;
          pbVar19 = pbVar19 + 1;
          puVar10 = (uint *)((uint)*pbVar1 * 0x40 + *(int *)0xc210);
          iVar14 = 4;
          do {
            *puVar23 = *puVar23 & puVar10[0x109b];
            puVar4 = puVar10;
            puVar10 = puVar10 + 1;
            *puVar23 = *puVar23 | *puVar4;
            puVar23 = puVar23 + 1;
            iVar14 = iVar14 + -1;
          } while (iVar14 != 0);
          iVar15 = iVar15 + -1;
        } while (iVar15 != 0);
        *(int *)0xc210 = *(int *)0xc210 + 8;
        iVar13 = iVar13 + -1;
      } while (iVar13 != 0);
      pbVar18 = pbVar18 + *(int *)0xc204;
      iVar12 = iVar12 + -1;
    } while (iVar12 != 0);
  }
  else {
    do {
      *(undefined2 *)0xc210 = *(undefined2 *)0xc208;
      iVar13 = 8;
      do {
        iVar15 = *(int *)0xc1ea;
        pbVar19 = pbVar18;
        do {
          pbVar1 = pbVar19;
          pbVar19 = pbVar19 + 1;
          puVar10 = (uint *)((uint)*pbVar1 * 0x40 + *(int *)0xc210);
          for (iVar14 = 4; iVar14 != 0; iVar14 = iVar14 + -1) {
            puVar6 = puVar23;
            puVar23 = puVar23 + 1;
            puVar4 = puVar10;
            puVar10 = puVar10 + 1;
            *puVar6 = *puVar4;
          }
          iVar15 = iVar15 + -1;
        } while (iVar15 != 0);
        *(int *)0xc210 = *(int *)0xc210 + 8;
        iVar13 = iVar13 + -1;
      } while (iVar13 != 0);
      pbVar18 = pbVar18 + *(int *)0xc204;
      iVar12 = iVar12 + -1;
    } while (iVar12 != 0);
  }
  uVar17 = (uint)*(byte *)0xc496;
  if (uVar17 != 0) {
    iVar12 = 0;
    do {
      *(uint *)0xc202 = (uint)*(byte *)(iVar12 + -0x3dde);
      *(uint *)0xc200 = (uint)*(byte *)(iVar12 + -0x3ddd);
      *(int *)0xc21a = *(int *)(iVar12 + -0x3de2) - *(int *)0xc216;
      iVar13 = *(int *)(iVar12 + -0x3de0) - *(int *)0xc218;
      *(int *)0xc21c = iVar13;
      if ((((iVar13 < *(int *)0xc1ee) && (0 < iVar13 + *(int *)0xc200)) &&
          (*(int *)0xc21a < *(int *)0xc1ec)) && (0 < *(int *)0xc21a + *(int *)0xc202)) {
        if (((*(uint *)0xc21c < 0x8000) &&
            ((int)(*(uint *)0xc21c + *(int *)0xc200 + -1) < *(int *)0xc1ee)) &&
           ((-1 < *(int *)0xc21a && (*(int *)0xc21a + *(int *)0xc202 + -1 < *(int *)0xc1ec)))) {
          uVar7 = *(undefined2 *)0xc212;
          pcVar24 = (char *)(*(int *)0xc214 + *(int *)0xc21c * *(int *)0xc1ec + *(int *)0xc21a);
          pcVar20 = (char *)*(undefined2 *)(iVar12 + -0x3ddc);
          *(int *)0xc1e8 = *(int *)0xc1ec - *(int *)0xc202;
          iVar13 = *(int *)0xc200;
          iVar15 = *(int *)0xc202;
          iVar14 = *(int *)0xc1e8;
          uVar8 = *(undefined2 *)0xc1fa;
          iVar16 = iVar15;
          do {
            do {
              pcVar2 = pcVar20;
              pcVar20 = pcVar20 + 1;
              if (*pcVar2 != '\0') {
                *pcVar24 = *pcVar2;
              }
              pcVar24 = pcVar24 + 1;
              iVar16 = iVar16 + -1;
            } while (iVar16 != 0);
            pcVar24 = pcVar24 + iVar14;
            iVar13 = iVar13 + -1;
            iVar16 = iVar15;
          } while (iVar13 != 0);
        }
        else {
          *(undefined2 *)0xc210 = 0;
          uVar7 = *(undefined2 *)0xc212;
          iVar15 = *(int *)0xc214;
          pcVar20 = (char *)*(undefined2 *)(iVar12 + -0x3ddc);
          iVar13 = *(int *)0xc21c;
          if (iVar13 < 0) {
            iVar14 = *(int *)0xc202;
            *(int *)0xc200 = *(int *)0xc200 + iVar13;
            pcVar20 = pcVar20 + -iVar13 * iVar14;
          }
          iVar13 = *(int *)0xc21a;
          if (iVar13 < 0) {
            pcVar20 = pcVar20 + -iVar13;
            *(int *)0xc210 = -iVar13;
          }
          else if (*(int *)0xc1ec < iVar13 + *(int *)0xc202) {
            *(int *)0xc210 = (iVar13 + *(int *)0xc202) - *(int *)0xc1ec;
          }
          iVar13 = (*(int *)0xc21c + *(int *)0xc200) - *(int *)0xc1ee;
          if (-1 < iVar13) {
            *(int *)0xc200 = *(int *)0xc200 - iVar13;
          }
          *(int *)0xc202 = *(int *)0xc202 - *(int *)0xc210;
          if (*(int *)0xc21c < 0) {
            *(undefined2 *)0xc21c = 0;
          }
          if (*(int *)0xc21a < 0) {
            *(undefined2 *)0xc21a = 0;
          }
          pcVar24 = (char *)(iVar15 + *(int *)0xc21c * *(int *)0xc1ec + *(int *)0xc21a);
          *(int *)0xc1e8 = *(int *)0xc1ec - *(int *)0xc202;
          iVar13 = *(int *)0xc200;
          do {
            iVar15 = *(int *)0xc202;
            uVar8 = *(undefined2 *)0xc1fa;
            do {
              pcVar2 = pcVar20;
              pcVar20 = pcVar20 + 1;
              if (*pcVar2 != '\0') {
                *pcVar24 = *pcVar2;
              }
              pcVar24 = pcVar24 + 1;
              iVar15 = iVar15 + -1;
            } while (iVar15 != 0);
            pcVar24 = pcVar24 + *(int *)0xc1e8;
            pcVar20 = pcVar20 + *(int *)0xc210;
            iVar13 = iVar13 + -1;
          } while (iVar13 != 0);
        }
      }
      iVar12 = iVar12 + 8;
      uVar17 = uVar17 - 1;
    } while (uVar17 != 0);
  }
  return;
}



// ================================================
// Function: FUN_18ac_03c8 at 18ac:03c8
// ================================================

void __cdecl16far FUN_18ac_03c8(void)

{
  undefined2 *puVar1;
  undefined2 *puVar2;
  int iVar3;
  undefined2 uVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  undefined2 *puVar8;
  undefined2 *puVar9;
  undefined2 unaff_DS;
  
  puVar9 = (undefined2 *)*(undefined2 *)0xc1f4;
  puVar8 = (undefined2 *)(*(int *)0xc214 + *(int *)0xc1ec * *(int *)0xc20c + *(int *)0xc20a);
  uVar7 = *(int *)0xc1ec - 8U >> 1;
  iVar3 = *(int *)0xc206;
  iVar5 = *(int *)0xc1ee + -8;
  uVar4 = *(undefined2 *)0xc212;
  uVar6 = uVar7;
  do {
    for (; uVar6 != 0; uVar6 = uVar6 - 1) {
      puVar2 = puVar9;
      puVar9 = puVar9 + 1;
      puVar1 = puVar8;
      puVar8 = puVar8 + 1;
      *puVar2 = *puVar1;
    }
    puVar9 = (undefined2 *)((int)puVar9 + iVar3 + 8);
    puVar8 = puVar8 + 4;
    iVar5 = iVar5 + -1;
    uVar6 = uVar7;
  } while (iVar5 != 0);
  return;
}



// ================================================
// Function: FUN_18ac_0417 at 18ac:0417
// ================================================

void __cdecl16far FUN_18ac_0417(void)

{
  code *pcVar1;
  
  pcVar1 = (code *)swi(0x10);
  (*pcVar1)();
  pcVar1 = (code *)swi(0x10);
  (*pcVar1)();
  return;
}



// ================================================
// Function: FUN_18ac_0422 at 18ac:0422
// ================================================

void __stdcall16far FUN_18ac_0422(undefined4 param_1)

{
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  int local_186;
  undefined1 local_184;
  byte local_183;
  undefined1 local_182 [128];
  undefined1 local_102 [252];
  undefined2 uStack_6;
  undefined2 uStack_4;
  
  uStack_4 = 0x18ac;
  uStack_6 = 0x8eed;
  FUN_1920_04df();
  FUN_1920_09f4(0xff,local_102,unaff_SS,(int)param_1,(int)((ulong)param_1 >> 0x10));
  FUN_1920_15ca(local_102,unaff_SS,local_182,unaff_SS);
  FUN_1920_15f8(1,local_182,unaff_SS);
  FUN_1920_16e3(&local_186,unaff_SS,1,&local_184,unaff_SS,local_182,unaff_SS);
  FUN_1920_16e3(&local_186,unaff_SS,1,&local_183,unaff_SS,local_182,unaff_SS);
  FUN_1920_16e3(&local_186,unaff_SS,(uint)local_183 << 6,0x7f74,unaff_DS,local_182,unaff_SS);
  local_186 = 0;
  while( true ) {
    if (*(char *)(local_186 + 0x7f74) == '\0') {
      *(undefined1 *)(local_186 + -0x5f56) = 0xff;
    }
    else {
      *(undefined1 *)(local_186 + -0x5f56) = 0;
    }
    if (local_186 == (uint)local_183 << 6) break;
    local_186 = local_186 + 1;
  }
  FUN_1920_1679(local_182,unaff_SS);
  return;
}



// ================================================
// Function: FUN_18ac_0517 at 18ac:0517
// ================================================

void __stdcall16far FUN_18ac_0517(undefined2 param_1,undefined2 param_2,int param_3)

{
  int iVar1;
  undefined2 unaff_DS;
  undefined2 local_6;
  
  FUN_1920_04df();
  if (*(byte *)0xc496 < 0x21) {
    iVar1 = (uint)*(byte *)0xc496 * 8;
    _local_6 = (undefined2 *)CONCAT22(unaff_DS,(undefined2 *)(iVar1 + -0x3de2));
    *_local_6 = param_2;
    *(undefined2 *)(iVar1 + -0x3de0) = param_1;
    *(undefined2 *)(iVar1 + -0x3ddc) = *(undefined2 *)(param_3 * 4 + -0x3cdc);
    *(undefined1 *)(iVar1 + -0x3dde) = *(undefined1 *)(param_3 * 4 + -0x3cde);
    *(undefined1 *)(iVar1 + -0x3ddd) = *(undefined1 *)(param_3 * 4 + -0x3cdd);
    *(char *)0xc496 = *(char *)0xc496 + '\x01';
  }
  return;
}



// ================================================
// Function: FUN_18ac_0594 at 18ac:0594
// ================================================

void __stdcall16far FUN_18ac_0594(int param_1)

{
  undefined1 *puVar1;
  undefined1 *puVar2;
  int iVar3;
  undefined1 *puVar4;
  undefined1 *puVar5;
  undefined2 unaff_DS;
  
  FUN_1920_04df();
  if (*(char *)0xc496 != '\0') {
    if (param_1 < (int)(*(byte *)0xc496 - 1)) {
      puVar5 = (undefined1 *)(*(int *)0xc1fc + param_1 * 8);
      puVar4 = puVar5 + 8;
      for (iVar3 = (((uint)*(byte *)0xc496 - param_1) + -1) * 8; iVar3 != 0; iVar3 = iVar3 + -1) {
        puVar2 = puVar5;
        puVar5 = puVar5 + 1;
        puVar1 = puVar4;
        puVar4 = puVar4 + 1;
        *puVar2 = *puVar1;
      }
    }
    *(char *)0xc496 = *(char *)0xc496 + -1;
  }
  return;
}



// ================================================
// Function: FUN_18ac_05dc at 18ac:05dc
// ================================================

void __stdcall16far FUN_18ac_05dc(undefined4 param_1)

{
  uint uVar1;
  uint uVar2;
  undefined2 unaff_SS;
  undefined2 unaff_DS;
  undefined1 local_18c [2];
  int local_18a;
  byte local_187;
  byte local_186;
  byte local_185;
  uint local_184;
  undefined1 local_182 [128];
  undefined1 local_102 [252];
  undefined2 uStack_6;
  undefined2 uStack_4;
  
  uStack_4 = 0x18ac;
  uStack_6 = 0x90a7;
  FUN_1920_04df();
  FUN_1920_09f4(0xff,local_102,unaff_SS,(int)param_1,(int)((ulong)param_1 >> 0x10));
  FUN_1920_15ca(local_102,unaff_SS,local_182,unaff_SS);
  FUN_1920_15f8(1,local_182,unaff_SS);
  local_18a = 0;
  FUN_1920_16e3(local_18c,unaff_SS,1,&local_185,unaff_SS,local_182,unaff_SS);
  if (local_185 != 0) {
    local_184 = 1;
    while( true ) {
      FUN_1920_16e3(local_18c,unaff_SS,1,&local_186,unaff_SS,local_182,unaff_SS);
      FUN_1920_16e3(local_18c,unaff_SS,1,&local_187,unaff_SS,local_182,unaff_SS);
      *(byte *)(local_184 * 4 + -0x3cde) = local_186;
      *(byte *)(local_184 * 4 + -0x3cdd) = local_187;
      uVar2 = (uint)local_187;
      uVar1 = (uint)local_186;
      FUN_1920_16e3(local_18c,unaff_SS,uVar1 * uVar2,(int)*(undefined4 *)0xc1e4 + local_18a,
                    (int)((ulong)*(undefined4 *)0xc1e4 >> 0x10),local_182,unaff_SS);
      *(int *)(local_184 * 4 + -0x3cdc) = (int)*(undefined4 *)0xc1e4 + local_18a;
      local_18a = local_18a + uVar1 * uVar2;
      if (local_184 == local_185) break;
      local_184 = local_184 + 1;
    }
  }
  FUN_1920_1679(local_182,unaff_SS);
  return;
}



// ================================================
// Function: FUN_18ac_072d at 18ac:072d
// ================================================

void __cdecl16far FUN_18ac_072d(void)

{
  FUN_1920_04df();
  return;
}



// ================================================
// Function: FUN_1920_0000 at 1920:0000
// ================================================

void __cdecl16far FUN_1920_0000(void)

{
  code *pcVar1;
  int iVar2;
  undefined2 in_BX;
  undefined2 *puVar3;
  undefined2 unaff_ES;
  int unaff_SS;
  
  DAT_1aa2_1aca = ((uint)&stack0x0013 >> 4) + unaff_SS;
  DAT_1aa2_1ace = DAT_1aa2_1aca + DAT_1aa2_1ac4;
  DAT_1aa2_1ae0 = *(undefined2 *)0x2;
  DAT_1aa2_1aea = 0xa9;
  DAT_1aa2_1aec = 0x1920;
  puVar3 = (undefined2 *)0xc69e;
  iVar2 = 0x13;
  DAT_1aa2_1acc = DAT_1aa2_1aca;
  DAT_1aa2_1ad8 = DAT_1aa2_1ace;
  DAT_1aa2_1adc = DAT_1aa2_1ace;
  DAT_1aa2_1ae4 = DAT_1aa2_1ace;
  DAT_1aa2_1af8 = unaff_ES;
  do {
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
    *puVar3 = in_BX;
    puVar3[1] = unaff_ES;
    puVar3 = puVar3 + 2;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  FUN_1920_04f7();
  FUN_1920_0567();
  FUN_1920_04f7();
  FUN_1920_056c();
  return;
}



// ================================================
// Function: FUN_1920_00e2 at 1920:00e2
// ================================================

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Restarted to delay deadcode elimination for space: ram */

void __cdecl16far FUN_1920_00e2(void)

{
  code *pcVar1;
  undefined2 in_AX;
  int iVar2;
  int iVar3;
  char *pcVar4;
  int in_stack_00000000;
  int in_stack_00000002;
  
  DAT_1aa2_1af2 = in_AX;
  iVar2 = DAT_1aa2_1ad0;
  if (in_stack_00000000 != 0 || in_stack_00000002 != 0) {
    for (; (iVar3 = in_stack_00000002, iVar2 != 0 &&
           (iVar3 = iVar2, in_stack_00000002 != *(int *)0x10)); iVar2 = *(int *)0x14) {
    }
    in_stack_00000002 = (iVar3 - DAT_1aa2_1af8) + -0x10;
  }
  DAT_1aa2_1af4 = in_stack_00000000;
  DAT_1aa2_1af6 = in_stack_00000002;
  pcVar4 = (char *)_DAT_1aa2_1aee;
  if ((int)((ulong)_DAT_1aa2_1aee >> 0x10) == 0 && pcVar4 == (char *)0x0) {
    FUN_1920_05c1();
    FUN_1920_05c1();
    iVar2 = 0x13;
    do {
      pcVar1 = (code *)swi(0x21);
      (*pcVar1)();
      iVar2 = iVar2 + -1;
    } while (iVar2 != 0);
    if (DAT_1aa2_1af4 != 0 || DAT_1aa2_1af6 != 0) {
      FUN_1920_01a5();
      FUN_1920_01b3();
      FUN_1920_01a5();
      FUN_1920_01cd();
      FUN_1920_01e7();
      FUN_1920_01cd();
      pcVar4 = (char *)0x215;
      FUN_1920_01a5();
    }
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
    for (; *pcVar4 != '\0'; pcVar4 = pcVar4 + 1) {
      FUN_1920_01e7();
    }
    return;
  }
  _DAT_1aa2_1aee = (char *)0x0;
  DAT_1aa2_1afc = 0;
  return;
}



// ================================================
// Function: FUN_1920_00e9 at 1920:00e9
// ================================================

/* WARNING: Removing unreachable block (ram,0x000192fc) */
/* WARNING: Removing unreachable block (ram,0x000192ff) */
/* WARNING: Removing unreachable block (ram,0x00019303) */
/* WARNING: Removing unreachable block (ram,0x00019312) */
/* WARNING: Removing unreachable block (ram,0x0001930c) */
/* WARNING: Removing unreachable block (ram,0x00019314) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl16far FUN_1920_00e9(void)

{
  code *pcVar1;
  undefined2 in_AX;
  int iVar2;
  char *pcVar3;
  undefined2 uVar4;
  
  uVar4 = 0x1aa2;
  DAT_1aa2_1af2 = in_AX;
  DAT_1aa2_1af4 = 0;
  DAT_1aa2_1af6 = 0;
  pcVar3 = (char *)_DAT_1aa2_1aee;
  if ((int)((ulong)_DAT_1aa2_1aee >> 0x10) == 0 && pcVar3 == (char *)0x0) {
    FUN_1920_05c1(0xc49e,0x1aa2);
    FUN_1920_05c1(0xc59e,0x1aa2);
    iVar2 = 0x13;
    do {
      pcVar1 = (code *)swi(0x21);
      (*pcVar1)();
      iVar2 = iVar2 + -1;
    } while (iVar2 != 0);
    if (*(int *)0x1af4 != 0 || *(int *)0x1af6 != 0) {
      FUN_1920_01a5();
      FUN_1920_01b3();
      FUN_1920_01a5();
      FUN_1920_01cd();
      FUN_1920_01e7();
      FUN_1920_01cd();
      pcVar3 = (char *)0x215;
      FUN_1920_01a5();
    }
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
    for (; *pcVar3 != '\0'; pcVar3 = pcVar3 + 1) {
      FUN_1920_01e7();
    }
    return;
  }
  _DAT_1aa2_1aee = (char *)0x0;
  DAT_1aa2_1afc = 0;
  return;
}



// ================================================
// Function: FUN_1920_01a5 at 1920:01a5
// ================================================

void __cdecl16near FUN_1920_01a5(void)

{
  char *in_BX;
  
  for (; *in_BX != '\0'; in_BX = in_BX + 1) {
    FUN_1920_01e7();
  }
  return;
}



// ================================================
// Function: FUN_1920_01b3 at 1920:01b3
// ================================================

undefined2 __cdecl16near FUN_1920_01b3(void)

{
  undefined2 uVar1;
  undefined1 uVar2;
  
  FUN_1920_01bf();
  uVar1 = FUN_1920_01bf();
  uVar1 = CONCAT11((char)((uint)uVar1 >> 8),(char)uVar1 + '0');
  FUN_1920_01e7(uVar1);
  uVar2 = (undefined1)((uint)uVar1 >> 8);
  return CONCAT11(uVar2,uVar2);
}



// ================================================
// Function: FUN_1920_01bf at 1920:01bf
// ================================================

undefined2 __cdecl16near FUN_1920_01bf(void)

{
  uint in_AX;
  undefined2 uVar1;
  undefined1 uVar2;
  byte in_CL;
  
  uVar1 = CONCAT11((char)((in_AX & 0xff) % (uint)in_CL),(char)((in_AX & 0xff) / (uint)in_CL) + '0');
  FUN_1920_01e7(uVar1);
  uVar2 = (undefined1)((uint)uVar1 >> 8);
  return CONCAT11(uVar2,uVar2);
}



// ================================================
// Function: FUN_1920_01cd at 1920:01cd
// ================================================

void FUN_1920_01cd(void)

{
  code *pcVar1;
  
  FUN_1920_01d4();
  FUN_1920_01df();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  return;
}



// ================================================
// Function: FUN_1920_01d4 at 1920:01d4
// ================================================

void FUN_1920_01d4(void)

{
  code *pcVar1;
  
  FUN_1920_01df();
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  return;
}



// ================================================
// Function: FUN_1920_01df at 1920:01df
// ================================================

void FUN_1920_01df(void)

{
  code *pcVar1;
  
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  return;
}



// ================================================
// Function: FUN_1920_01e7 at 1920:01e7
// ================================================

void __cdecl16near FUN_1920_01e7(void)

{
  code *pcVar1;
  
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  return;
}



// ================================================
// Function: FUN_1920_023f at 1920:023f
// ================================================

void __stdcall16far FUN_1920_023f(void)

{
  undefined1 in_CF;
  
  FUN_1920_0308();
  if (!(bool)in_CF) {
    return;
  }
  FUN_1920_00e2();
  return;
}



// ================================================
// Function: FUN_1920_0254 at 1920:0254
// ================================================

void __stdcall16far FUN_1920_0254(void)

{
  undefined1 in_CF;
  
  FUN_1920_03d3();
  if (!(bool)in_CF) {
    return;
  }
  FUN_1920_00e2();
  return;
}



// ================================================
// Function: FUN_1920_02b8 at 1920:02b8
// ================================================

int __cdecl16far FUN_1920_02b8(void)

{
  undefined4 uVar1;
  
  uVar1 = FUN_1000_02d1();
  return (int)uVar1 + (int)((ulong)uVar1 >> 0x10) * 0x10;
}



// ================================================
// Function: FUN_1920_0308 at 1920:0308
// ================================================

uint * __cdecl16near FUN_1920_0308(void)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  uint *puVar4;
  byte bVar5;
  int in_AX;
  undefined2 *puVar6;
  uint uVar7;
  uint uVar8;
  uint uVar9;
  uint *puVar10;
  int iVar11;
  int iVar12;
  int unaff_DS;
  bool bVar13;
  undefined4 uVar14;
  uint *puVar15;
  
  if (in_AX != 0) {
    *(int *)0xc6ea = in_AX;
    do {
      uVar14 = FUN_1920_0491();
      uVar9 = (uint)((ulong)uVar14 >> 0x10);
      uVar7 = (uint)uVar14;
      puVar10 = (uint *)0x1ae2;
      iVar12 = unaff_DS;
      while( true ) {
        iVar11 = iVar12;
        puVar15 = puVar10;
        puVar4 = *(uint **)puVar15;
        iVar12 = (int)((ulong)puVar4 >> 0x10);
        puVar10 = (uint *)puVar4;
        if (iVar12 == *(int *)0x1adc) break;
        uVar1 = puVar10[3];
        bVar13 = uVar9 == uVar1;
        if ((uVar9 <= uVar1) &&
           ((uVar9 < uVar1 || (bVar13 = uVar7 == puVar10[2], uVar7 <= puVar10[2])))) {
          uVar1 = puVar10[1];
          puVar6 = (undefined2 *)*puVar4;
          uVar8 = uVar1;
          if (!bVar13) {
            uVar2 = puVar10[2];
            uVar3 = puVar10[3];
            bVar5 = (byte)(uVar7 + (int)puVar10);
            uVar8 = uVar9 + iVar12 + (uint)(0xf < bVar5);
            puVar6 = (undefined2 *)
                     (CONCAT11((char)(uVar7 + (int)puVar10 >> 8),bVar5 - 0x10) & 0xff0f);
            *puVar6 = (undefined2 *)*puVar4;
            puVar6[1] = uVar1;
            puVar6[2] = uVar2 - uVar7 & 0xf;
            puVar6[3] = (uVar3 - uVar9) - (uint)(uVar2 < uVar7);
          }
          *puVar15 = (uint)puVar6;
          puVar15[1] = uVar8;
          return puVar10;
        }
      }
      bVar5 = (byte)(uVar7 + (int)puVar10);
      uVar9 = uVar9 + iVar12 + (uint)(0xf < bVar5);
      uVar7 = CONCAT11((char)(uVar7 + (int)puVar10 >> 8),bVar5 - 0x10) & 0xff0f;
      if ((uVar9 < *(uint *)0x1ae0) || ((uVar9 <= *(uint *)0x1ae0 && (uVar7 <= *(uint *)0x1ade)))) {
        *(uint *)0x1ada = uVar7;
        *(uint *)0x1adc = uVar9;
        *puVar15 = uVar7;
        puVar15[1] = uVar9;
        puVar15 = (uint *)0x0;
        (*(code *)*(undefined2 *)0x1aea)(0x1920,0,puVar10,iVar12);
        return puVar15;
      }
      puVar10 = (uint *)(*(code *)*(undefined2 *)0x1aea)(0x1920,*(undefined2 *)0xc6ea);
      if ((char)puVar10 == '\0') {
        return puVar10;
      }
    } while ((char)puVar10 != '\x01');
  }
  return (uint *)0x0;
}



// ================================================
// Function: FUN_1920_03d3 at 1920:03d3
// ================================================

void __cdecl16near FUN_1920_03d3(void)

{
  undefined4 *puVar1;
  int in_AX;
  undefined4 *in_CX;
  uint in_BX;
  undefined4 *puVar2;
  undefined4 *puVar3;
  uint uVar4;
  uint unaff_DS;
  undefined4 uVar5;
  
  if ((((in_AX != 0) && (((uint)in_CX & 0xfff7) == 0)) &&
      ((*(uint *)0x1ad8 < in_BX ||
       ((*(uint *)0x1ad8 <= in_BX && ((undefined4 *)*(uint *)0x1ad6 <= in_CX)))))) &&
     ((in_BX < *(uint *)0x1adc ||
      ((in_BX <= *(uint *)0x1adc && (in_CX < (undefined4 *)*(uint *)0x1ada)))))) {
    uVar5 = FUN_1920_0491();
    *(int *)(in_CX + 1) = (int)uVar5;
    *(undefined2 *)((int)in_CX + 6) = (int)((ulong)uVar5 >> 0x10);
    puVar3 = (undefined4 *)0x1ae2;
    do {
      do {
        uVar4 = unaff_DS;
        puVar2 = puVar3;
        puVar1 = (undefined4 *)*puVar2;
        unaff_DS = (uint)((ulong)puVar1 >> 0x10);
        puVar3 = (undefined4 *)puVar1;
      } while (unaff_DS < in_BX);
      if (in_BX < unaff_DS) goto LAB_1920_0424;
    } while (puVar3 < in_CX);
    if (in_CX != puVar3) {
LAB_1920_0424:
      FUN_1920_0432(puVar2,uVar4);
      FUN_1920_0432();
      return;
    }
  }
  return;
}



// ================================================
// Function: FUN_1920_0432 at 1920:0432
// ================================================

void __cdecl16near FUN_1920_0432(void)

{
  int iVar1;
  int iVar2;
  byte bVar3;
  int iVar4;
  undefined2 *in_CX;
  int in_BX;
  undefined2 *unaff_DI;
  int unaff_ES;
  undefined2 unaff_DS;
  
  bVar3 = (byte)((int)in_CX + in_CX[2]);
  if ((in_BX + in_CX[3] + (uint)(0xf < bVar3) != unaff_ES) ||
     ((undefined2 *)(CONCAT11((char)((uint)((int)in_CX + in_CX[2]) >> 8),bVar3 - 0x10) & 0xff0f) !=
      unaff_DI)) {
    *in_CX = unaff_DI;
    in_CX[1] = unaff_ES;
    return;
  }
  if ((unaff_ES == *(int *)0x1adc) && (unaff_DI == (undefined2 *)*(undefined2 *)0x1ada)) {
    *(undefined2 *)0x1ada = in_CX;
    *(int *)0x1adc = in_BX;
    return;
  }
  *in_CX = *unaff_DI;
  in_CX[1] = unaff_DI[1];
  iVar1 = unaff_DI[3];
  iVar4 = unaff_DI[2] + in_CX[2];
  iVar2 = in_CX[3];
  bVar3 = (byte)iVar4;
  in_CX[2] = CONCAT11((char)((uint)iVar4 >> 8),bVar3 - 0x10) & 0xff0f;
  in_CX[3] = iVar1 + iVar2 + (uint)(0xf < bVar3);
  return;
}



// ================================================
// Function: FUN_1920_0491 at 1920:0491
// ================================================

ulong __cdecl16near FUN_1920_0491(void)

{
  uint in_AX;
  
  return CONCAT22((in_AX + 7 >> 1 | (uint)(0xfff8 < in_AX) << 0xf) >> 3,in_AX + 7) & 0xffff0008;
}



// ================================================
// Function: FUN_1920_04a2 at 1920:04a2
// ================================================

undefined2 __cdecl16far FUN_1920_04a2(void)

{
  undefined2 uVar1;
  undefined2 unaff_DS;
  
  LOCK();
  uVar1 = *(undefined2 *)0x1afc;
  *(undefined2 *)0x1afc = 0;
  UNLOCK();
  return uVar1;
}



// ================================================
// Function: FUN_1920_04a9 at 1920:04a9
// ================================================

void __cdecl16far FUN_1920_04a9(void)

{
  undefined2 unaff_DS;
  
  if (*(int *)0x1afc == 0) {
    return;
  }
  FUN_1920_00e2();
  return;
}



// ================================================
// Function: FUN_1920_04df at 1920:04df
// ================================================

void __cdecl16far FUN_1920_04df(void)

{
  uint in_AX;
  undefined2 unaff_DS;
  
  if (((in_AX < 0xfe00) && ((undefined1 *)(in_AX + 0x200) < &stack0x0000)) &&
     (*(uint *)0x1afa <= (uint)-((int)(in_AX + 0x200) - (int)&stack0x0000))) {
    return;
  }
  FUN_1920_00e2();
  return;
}



// ================================================
// Function: FUN_1920_04f7 at 1920:04f7
// ================================================

void __stdcall16far FUN_1920_04f7(byte *param_1,undefined2 *param_2)

{
  byte *pbVar1;
  byte bVar2;
  int iVar3;
  uint uVar4;
  byte *pbVar5;
  undefined2 *puVar6;
  byte *pbVar7;
  undefined2 uVar8;
  
  uVar8 = (undefined2)((ulong)param_2 >> 0x10);
  puVar6 = (undefined2 *)param_2;
  pbVar5 = (byte *)param_1;
  *param_2 = 0;
  puVar6[1] = 0xd7b0;
  puVar6[2] = 0x80;
  puVar6[3] = 0;
  puVar6[4] = 0;
  puVar6[5] = 0;
  puVar6[6] = puVar6 + 0x40;
  puVar6[7] = uVar8;
  pbVar7 = (byte *)(puVar6 + 9);
  puVar6[8] = 0x60e;
  pbVar7[0] = 0x20;
  pbVar7[1] = 0x19;
  for (iVar3 = 0xe; pbVar7 = pbVar7 + 2, iVar3 != 0; iVar3 = iVar3 + -1) {
    pbVar7[0] = 0;
    pbVar7[1] = 0;
  }
  bVar2 = *param_1;
  if (0x4f < bVar2) {
    bVar2 = 0x4f;
  }
  for (uVar4 = (uint)bVar2; pbVar5 = pbVar5 + 1, uVar4 != 0; uVar4 = uVar4 - 1) {
    pbVar1 = pbVar7;
    pbVar7 = pbVar7 + 1;
    *pbVar1 = *pbVar5;
  }
  *pbVar7 = 0;
  return;
}



// ================================================
// Function: FUN_1920_0567 at 1920:0567
// ================================================

void FUN_1920_0567(void)

{
  FUN_1920_0574();
  return;
}



// ================================================
// Function: FUN_1920_056c at 1920:056c
// ================================================

void FUN_1920_056c(void)

{
  FUN_1920_0574();
  return;
}



// ================================================
// Function: FUN_1920_0574 at 1920:0574
// ================================================

void __stdcall16far FUN_1920_0574(undefined4 param_1)

{
  int iVar1;
  undefined2 in_DX;
  int iVar2;
  undefined2 uVar3;
  undefined2 unaff_DS;
  bool bVar4;
  
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  iVar1 = *(int *)(iVar2 + 2);
  if ((iVar1 == -0x284f) || (iVar1 == -0x284e)) {
    FUN_1920_05c1(iVar2,uVar3);
  }
  else if (iVar1 != -0x2850) {
    *(undefined2 *)0x1afc = 0x66;
    return;
  }
  bVar4 = true;
  *(undefined2 *)(iVar2 + 2) = in_DX;
  *(undefined2 *)(iVar2 + 8) = 0;
  *(undefined2 *)(iVar2 + 10) = 0;
  FUN_1920_05fd();
  if (!bVar4) {
    *(undefined2 *)(iVar2 + 2) = 0xd7b0;
  }
  return;
}



// ================================================
// Function: FUN_1920_05c1 at 1920:05c1
// ================================================

void __stdcall16far FUN_1920_05c1(undefined4 param_1)

{
  int iVar1;
  undefined2 uVar2;
  undefined2 unaff_DS;
  
  uVar2 = (undefined2)((ulong)param_1 >> 0x10);
  iVar1 = (int)param_1;
  if (*(int *)(iVar1 + 2) != -0x284f) {
    if (*(int *)(iVar1 + 2) != -0x284e) {
      *(undefined2 *)0x1afc = 0x67;
      return;
    }
    FUN_1920_05fd();
  }
  FUN_1920_05fd();
  *(undefined2 *)(iVar1 + 2) = 0xd7b0;
  return;
}



// ================================================
// Function: FUN_1920_05fd at 1920:05fd
// ================================================

void __cdecl16near FUN_1920_05fd(void)

{
  int iVar1;
  int in_BX;
  int unaff_DI;
  undefined2 unaff_DS;
  
  iVar1 = (*(code *)*(undefined2 *)(in_BX + unaff_DI))(0x1920);
  if (iVar1 != 0) {
    *(int *)0x1afc = iVar1;
  }
  return;
}



// ================================================
// Function: FUN_1920_0778 at 1920:0778
// ================================================

void __cdecl16near FUN_1920_0778(void)

{
  int unaff_DI;
  undefined2 unaff_ES;
  undefined2 unaff_DS;
  
  if ((*(int *)0x1afc == 0) && (*(int *)(unaff_DI + 2) != -0x284f)) {
    *(undefined2 *)0x1afc = 0x68;
  }
  return;
}



// ================================================
// Function: FUN_1920_079c at 1920:079c
// ================================================

undefined1 __cdecl16near FUN_1920_079c(undefined4 param_1)

{
  int iVar1;
  int in_DX;
  int in_BX;
  int unaff_DI;
  undefined2 unaff_ES;
  
  if (in_BX == in_DX) {
    unaff_ES = (undefined2)((ulong)param_1 >> 0x10);
    unaff_DI = (int)param_1;
    iVar1 = FUN_1920_07eb();
    if (in_BX == iVar1) {
      return 0x1a;
    }
  }
  return *(undefined1 *)(in_BX + unaff_DI);
}



// ================================================
// Function: FUN_1920_07b8 at 1920:07b8
// ================================================

void __cdecl16near FUN_1920_07b8(void)

{
  int unaff_DI;
  undefined2 unaff_ES;
  undefined2 unaff_DS;
  
  if ((*(int *)0x1afc == 0) && (*(int *)(unaff_DI + 2) != -0x284e)) {
    *(undefined2 *)0x1afc = 0x69;
  }
  return;
}



// ================================================
// Function: FUN_1920_07dc at 1920:07dc
// ================================================

undefined4 __cdecl16near FUN_1920_07dc(undefined4 param_1)

{
  undefined2 in_AX;
  int iVar1;
  int in_DX;
  int in_BX;
  int unaff_DI;
  int iVar2;
  undefined2 unaff_ES;
  undefined2 uVar3;
  
  *(undefined1 *)(in_BX + unaff_DI) = (char)in_AX;
  if (in_BX + 1 != in_DX) {
    return CONCAT22(in_DX,in_AX);
  }
  uVar3 = (undefined2)((ulong)param_1 >> 0x10);
  iVar2 = (int)param_1;
  *(int *)(iVar2 + 8) = in_BX + 1;
  iVar1 = (*(code *)*(undefined2 *)(iVar2 + 0x14))(0x1920,iVar2,uVar3,iVar2,uVar3);
  if (iVar1 != 0) {
    DAT_1aa2_1afc = iVar1;
  }
  return CONCAT22(*(undefined2 *)(iVar2 + 4),*(undefined2 *)(iVar2 + 10));
}



// ================================================
// Function: FUN_1920_07eb at 1920:07eb
// ================================================

undefined4 __cdecl16near FUN_1920_07eb(void)

{
  int iVar1;
  undefined2 in_BX;
  int unaff_DI;
  undefined2 unaff_ES;
  
  *(undefined2 *)(unaff_DI + 8) = in_BX;
  iVar1 = (*(code *)*(undefined2 *)(unaff_DI + 0x14))(0x1920);
  if (iVar1 != 0) {
    DAT_1aa2_1afc = iVar1;
  }
  return CONCAT22(*(undefined2 *)(unaff_DI + 4),*(undefined2 *)(unaff_DI + 10));
}



// ================================================
// Function: FUN_1920_081c at 1920:081c
// ================================================

void __stdcall16far FUN_1920_081c(undefined4 param_1)

{
  char cVar1;
  int in_BX;
  undefined1 in_ZF;
  
  FUN_1920_0778();
  if ((bool)in_ZF) {
    do {
      cVar1 = FUN_1920_079c();
      if (cVar1 == '\x1a') goto LAB_1920_083d;
      in_BX = in_BX + 1;
    } while (cVar1 != '\r');
    cVar1 = FUN_1920_079c();
    if (cVar1 == '\n') {
      in_BX = in_BX + 1;
    }
  }
LAB_1920_083d:
  *(int *)((int)param_1 + 8) = in_BX;
  FUN_1920_087b();
  return;
}



// ================================================
// Function: FUN_1920_084a at 1920:084a
// ================================================

void __stdcall16far FUN_1920_084a(undefined4 param_1)

{
  undefined2 in_BX;
  undefined1 in_ZF;
  
  FUN_1920_07b8();
  if ((bool)in_ZF) {
    FUN_1920_07dc();
    FUN_1920_07dc();
  }
  *(undefined2 *)((int)param_1 + 8) = in_BX;
  FUN_1920_087b();
  return;
}



// ================================================
// Function: FUN_1920_086e at 1920:086e
// ================================================

void __stdcall16far FUN_1920_086e(void)

{
  FUN_1920_087b();
  return;
}



// ================================================
// Function: FUN_1920_087b at 1920:087b
// ================================================

void __cdecl16near FUN_1920_087b(void)

{
  int iVar1;
  int unaff_DI;
  undefined2 unaff_ES;
  undefined2 unaff_DS;
  
  if (*(int *)(unaff_DI + 0x1a) == 0) {
    return;
  }
  if (*(int *)0x1afc == 0) {
    iVar1 = (*(code *)*(undefined2 *)(unaff_DI + 0x18))(0x1920);
    if (iVar1 != 0) {
      *(int *)0x1afc = iVar1;
    }
  }
  return;
}



// ================================================
// Function: FUN_1920_0898 at 1920:0898
// ================================================

void __stdcall16far FUN_1920_0898(int param_1,char *param_2,undefined4 param_3)

{
  char cVar1;
  int iVar2;
  int in_BX;
  char *pcVar3;
  undefined2 uVar4;
  undefined1 in_ZF;
  
  FUN_1920_0778();
  uVar4 = (undefined2)((ulong)param_2 >> 0x10);
  pcVar3 = (char *)param_2;
  iVar2 = 0;
  if ((bool)in_ZF) {
    do {
      cVar1 = FUN_1920_079c();
      if ((cVar1 == '\r') || (cVar1 == '\x1a')) break;
      in_BX = in_BX + 1;
      iVar2 = iVar2 + 1;
      pcVar3 = pcVar3 + 1;
      *pcVar3 = cVar1;
    } while (iVar2 != param_1);
  }
  pcVar3[-iVar2] = (char)iVar2;
  *(int *)((int)param_3 + 8) = in_BX;
  return;
}



// ================================================
// Function: FUN_1920_08d0 at 1920:08d0
// ================================================

void __stdcall16far FUN_1920_08d0(int param_1,byte *param_2,undefined4 param_3)

{
  int iVar1;
  uint uVar2;
  undefined2 in_BX;
  undefined1 in_ZF;
  
  FUN_1920_07b8();
  if ((bool)in_ZF) {
    uVar2 = (uint)*param_2;
    iVar1 = param_1 - uVar2;
    if (iVar1 != 0 && (int)uVar2 <= param_1) {
      do {
        FUN_1920_07dc();
        iVar1 = iVar1 + -1;
      } while (iVar1 != 0);
      uVar2 = (uint)*param_2;
    }
    for (; uVar2 != 0; uVar2 = uVar2 - 1) {
      FUN_1920_07dc();
    }
  }
  *(undefined2 *)((int)param_3 + 8) = in_BX;
  return;
}



// ================================================
// Function: FUN_1920_090e at 1920:090e
// ================================================

void __stdcall16far FUN_1920_090e(int param_1,undefined1 *param_2,undefined1 *param_3)

{
  undefined1 *puVar1;
  undefined1 *puVar2;
  undefined1 *puVar3;
  undefined1 *puVar4;
  
  puVar3 = (undefined1 *)param_3;
  puVar4 = (undefined1 *)param_2;
  for (; param_1 != 0; param_1 = param_1 + -1) {
    puVar2 = puVar4;
    puVar4 = puVar4 + 1;
    puVar1 = puVar3;
    puVar3 = puVar3 + 1;
    *puVar2 = *puVar1;
  }
  return;
}



// ================================================
// Function: FUN_1920_092a at 1920:092a
// ================================================

int __cdecl16far FUN_1920_092a(void)

{
  int in_AX;
  int in_CX;
  
  return in_AX * in_CX;
}



// ================================================
// Function: FUN_1920_0945 at 1920:0945
// ================================================

uint __cdecl16far FUN_1920_0945(void)

{
  uint uVar1;
  uint in_AX;
  uint in_CX;
  uint uVar2;
  uint uVar3;
  uint in_DX;
  uint in_BX;
  uint uVar4;
  uint uVar5;
  int iVar6;
  bool bVar7;
  bool bVar8;
  bool bVar9;
  bool bVar10;
  
  if (in_CX == 0 && in_BX == 0) {
    uVar2 = FUN_1920_00e2();
    return uVar2;
  }
  bVar9 = (int)in_BX < 0;
  if (bVar9) {
    uVar2 = ~in_CX;
    in_CX = uVar2 + 1;
    in_BX = ~in_BX + (uint)(0xfffe < uVar2);
  }
  bVar10 = (int)in_DX < 0;
  if (bVar10) {
    uVar2 = ~in_AX;
    in_AX = uVar2 + 1;
    in_DX = ~in_DX + (uint)(0xfffe < uVar2);
  }
  uVar2 = 0;
  bVar7 = false;
  uVar4 = 0;
  iVar6 = 0x21;
  do {
    uVar3 = uVar2 << 1 | (uint)bVar7;
    uVar5 = uVar4 << 1 | (uint)((int)uVar2 < 0);
    uVar2 = uVar3 - in_CX;
    uVar3 = (uint)(uVar3 < in_CX);
    uVar1 = uVar5 - in_BX;
    bVar7 = uVar5 < in_BX || uVar1 < uVar3;
    uVar4 = uVar1 - uVar3;
    if (uVar5 < in_BX || uVar1 < uVar3) {
      bVar8 = CARRY2(uVar2,in_CX);
      uVar2 = uVar2 + in_CX;
      bVar7 = CARRY2(uVar4,in_BX) || CARRY2(uVar4 + in_BX,(uint)bVar8);
      uVar4 = uVar4 + in_BX + (uint)bVar8;
    }
    bVar8 = (int)in_AX < 0;
    in_AX = in_AX << 1 | (uint)!bVar7;
    bVar7 = (int)in_DX < 0;
    in_DX = in_DX << 1 | (uint)bVar8;
    iVar6 = iVar6 + -1;
  } while (iVar6 != 0);
  if (bVar10) {
    if (bVar9) {
      return in_AX;
    }
  }
  else if (!bVar9) {
    return in_AX;
  }
  return ~in_AX + 1;
}



// ================================================
// Function: FUN_1920_09da at 1920:09da
// ================================================

void __stdcall16far FUN_1920_09da(byte *param_1,byte *param_2)

{
  byte bVar1;
  uint uVar2;
  byte *pbVar3;
  byte *pbVar4;
  
  pbVar4 = (byte *)param_2;
  pbVar3 = (byte *)param_1;
  bVar1 = *param_1;
  *param_2 = bVar1;
  uVar2 = (uint)bVar1;
  while( true ) {
    pbVar4 = pbVar4 + 1;
    pbVar3 = pbVar3 + 1;
    if (uVar2 == 0) break;
    uVar2 = uVar2 - 1;
    *pbVar4 = *pbVar3;
  }
  return;
}



// ================================================
// Function: FUN_1920_09f4 at 1920:09f4
// ================================================

void __stdcall16far FUN_1920_09f4(byte param_1,byte *param_2,byte *param_3)

{
  byte bVar1;
  uint uVar2;
  byte *pbVar3;
  byte *pbVar4;
  
  pbVar3 = (byte *)param_3;
  pbVar4 = (byte *)param_2;
  bVar1 = *param_3;
  if (param_1 < *param_3) {
    bVar1 = param_1;
  }
  *param_2 = bVar1;
  uVar2 = (uint)bVar1;
  while( true ) {
    pbVar4 = pbVar4 + 1;
    pbVar3 = pbVar3 + 1;
    if (uVar2 == 0) break;
    uVar2 = uVar2 - 1;
    *pbVar4 = *pbVar3;
  }
  return;
}



// ================================================
// Function: FUN_1920_0a26 at 1920:0a26
// ================================================

void __stdcall16far FUN_1920_0a26(uint param_1,uint param_2,byte *param_3,byte *param_4)

{
  byte *pbVar1;
  uint uVar2;
  byte *pbVar3;
  byte *pbVar4;
  
  pbVar4 = (byte *)param_4;
  if ((int)param_2 < 1) {
    param_2 = 1;
  }
  pbVar3 = (byte *)param_3 + param_2;
  if (*param_3 < param_2) {
    uVar2 = 0;
  }
  else {
    uVar2 = (*param_3 - param_2) + 1;
    if ((int)param_1 < 0) {
      param_1 = 0;
    }
    if (param_1 < uVar2) {
      uVar2 = param_1;
    }
  }
  *param_4 = (byte)uVar2;
  for (; pbVar4 = pbVar4 + 1, uVar2 != 0; uVar2 = uVar2 - 1) {
    pbVar1 = pbVar3;
    pbVar3 = pbVar3 + 1;
    *pbVar4 = *pbVar1;
  }
  return;
}



// ================================================
// Function: FUN_1920_0a67 at 1920:0a67
// ================================================

void __stdcall16far FUN_1920_0a67(byte *param_1,byte *param_2)

{
  byte bVar1;
  byte bVar2;
  byte bVar3;
  uint uVar4;
  byte *pbVar5;
  byte *pbVar6;
  
  pbVar5 = (byte *)param_1;
  bVar2 = *param_2;
  bVar3 = *param_1;
  bVar1 = *param_2;
  *param_2 = *param_2 + bVar3;
  if (CARRY1(bVar1,bVar3)) {
    *param_2 = 0xff;
    bVar3 = ~bVar2;
  }
  pbVar6 = (byte *)param_2 + bVar2;
  uVar4 = (uint)bVar3;
  while( true ) {
    pbVar6 = pbVar6 + 1;
    pbVar5 = pbVar5 + 1;
    if (uVar4 == 0) break;
    uVar4 = uVar4 - 1;
    *pbVar6 = *pbVar5;
  }
  return;
}



// ================================================
// Function: FUN_1920_0adf at 1920:0adf
// ================================================

void __stdcall16far FUN_1920_0adf(byte *param_1,byte *param_2)

{
  byte bVar1;
  uint uVar2;
  byte *pbVar3;
  byte *pbVar4;
  
  pbVar3 = (byte *)param_2;
  pbVar4 = (byte *)param_1;
  bVar1 = *param_2;
  if (*param_1 < *param_2) {
    bVar1 = *param_1;
  }
  if (bVar1 != 0) {
    uVar2 = (uint)bVar1;
    do {
      pbVar4 = pbVar4 + 1;
      pbVar3 = pbVar3 + 1;
      if (uVar2 == 0) {
        return;
      }
      uVar2 = uVar2 - 1;
    } while (*pbVar3 == *pbVar4);
  }
  return;
}



// ================================================
// Function: FUN_1920_0b0a at 1920:0b0a
// ================================================

void __stdcall16far FUN_1920_0b0a(undefined1 param_1,undefined1 *param_2)

{
  *param_2 = 1;
  ((undefined1 *)param_2)[1] = param_1;
  return;
}



// ================================================
// Function: FUN_1920_0b37 at 1920:0b37
// ================================================

void __stdcall16far
FUN_1920_0b37(int param_1,undefined2 param_2,undefined4 param_3,undefined4 param_4)

{
  undefined2 uVar1;
  undefined2 uVar2;
  undefined2 unaff_SS;
  undefined1 *puVar3;
  undefined1 *puVar4;
  undefined2 uVar5;
  undefined1 local_202 [256];
  undefined1 local_102 [256];
  
  puVar4 = local_102;
  uVar2 = (undefined2)((ulong)param_3 >> 0x10);
  uVar1 = (undefined2)param_3;
  uVar5 = unaff_SS;
  FUN_1920_0a26(param_1 + -1,1,uVar1,uVar2);
  FUN_1920_0a67((int)param_4,(int)((ulong)param_4 >> 0x10));
  puVar3 = local_202;
  FUN_1920_0a26(0xff,param_1,uVar1,uVar2);
  FUN_1920_0a67(puVar3,unaff_SS);
  FUN_1920_09f4(param_2,uVar1,uVar2,puVar4,uVar5);
  return;
}



// ================================================
// Function: FUN_1920_0b8b at 1920:0b8b
// ================================================

void __stdcall16far FUN_1920_0b8b(int param_1,int param_2,undefined4 param_3)

{
  undefined2 uVar1;
  undefined2 uVar2;
  undefined2 unaff_SS;
  undefined1 *puVar3;
  undefined1 *puVar4;
  undefined2 uVar5;
  undefined1 local_202 [256];
  undefined1 local_102 [256];
  
  if (0 < param_1) {
    puVar4 = local_102;
    uVar2 = (undefined2)((ulong)param_3 >> 0x10);
    uVar1 = (undefined2)param_3;
    uVar5 = unaff_SS;
    FUN_1920_0a26(param_2 + -1,1,uVar1,uVar2);
    puVar3 = local_202;
    FUN_1920_0a26(0xff,param_2 + param_1,uVar1,uVar2);
    FUN_1920_0a67(puVar3,unaff_SS);
    FUN_1920_09f4(0xff,uVar1,uVar2,puVar4,uVar5);
  }
  return;
}



// ================================================
// Function: FUN_1920_0be1 at 1920:0be1
// ================================================

undefined4 FUN_1920_0be1(void)

{
  uint uVar1;
  byte bVar2;
  uint in_AX;
  uint uVar3;
  uint uVar4;
  char cVar5;
  uint in_CX;
  uint uVar6;
  uint in_DX;
  uint uVar7;
  uint in_BX;
  uint uVar8;
  uint uVar9;
  uint unaff_SI;
  uint unaff_DI;
  uint uVar10;
  uint uVar11;
  bool bVar12;
  bool bVar13;
  undefined4 uVar14;
  
  uVar10 = unaff_DI ^ 0x8000;
  if ((byte)in_CX != 0) {
    if ((byte)in_AX == 0) {
LAB_1920_0c65:
      return CONCAT22(uVar10,in_CX);
    }
    uVar4 = in_AX;
    uVar7 = in_DX;
    uVar8 = in_BX;
    if ((byte)in_CX < (byte)in_AX) {
      uVar4 = in_CX;
      in_CX = in_AX;
      uVar7 = uVar10;
      uVar8 = unaff_SI;
      unaff_SI = in_BX;
      uVar10 = in_DX;
    }
    bVar2 = -((char)uVar4 - (char)in_CX);
    if (0x27 < bVar2) goto LAB_1920_0c65;
    uVar3 = CONCAT11((char)(uVar7 >> 8),(char)in_CX) & 0x80ff;
    uVar4 = uVar4 & 0xff00;
    uVar7 = uVar7 | 0x8000;
    uVar11 = uVar10 | 0x8000;
    for (; 7 < bVar2; bVar2 = bVar2 - 8) {
      uVar4 = CONCAT11((char)uVar8,(char)(uVar4 >> 8));
      uVar8 = CONCAT11((char)uVar7,(char)(uVar8 >> 8));
      uVar7 = uVar7 >> 8;
    }
    uVar6 = CONCAT11((char)(in_CX >> 8),bVar2);
    while (bVar2 != 0) {
      uVar1 = uVar7 & 1;
      uVar7 = uVar7 >> 1;
      uVar9 = uVar8 & 1;
      uVar8 = uVar8 >> 1 | (uint)(uVar1 != 0) << 0xf;
      uVar4 = uVar4 >> 1 | (uint)(uVar9 != 0) << 0xf;
      bVar2 = (char)uVar6 - 1;
      uVar6 = CONCAT11((char)(uVar6 >> 8),bVar2);
    }
    if ((int)(uVar3 ^ uVar10) < 0) {
      in_AX = uVar4 - uVar6;
      uVar9 = (uVar8 - unaff_SI) - (uint)(uVar4 < uVar6);
      uVar10 = (uint)(uVar8 < unaff_SI || uVar8 - unaff_SI < (uint)(uVar4 < uVar6));
      in_DX = (uVar7 - uVar11) - uVar10;
      if (uVar7 < uVar11 || uVar7 - uVar11 < uVar10) {
        uVar10 = ~uVar9;
        bVar12 = in_AX == 0;
        in_AX = -in_AX;
        uVar9 = uVar10 + bVar12;
        in_DX = ~in_DX + (uint)CARRY2(uVar10,(uint)bVar12);
        uVar3 = uVar3 ^ 0x8000;
      }
      if ((in_DX == 0 && uVar9 == 0) && in_AX == 0) goto LAB_1920_0c64;
      while (-1 < (int)in_DX) {
        bVar12 = (int)in_AX < 0;
        in_AX = in_AX << 1;
        bVar13 = (int)uVar9 < 0;
        uVar9 = uVar9 << 1 | (uint)bVar12;
        in_DX = in_DX << 1 | (uint)bVar13;
        cVar5 = (char)uVar3 + -1;
        uVar3 = CONCAT11((char)(uVar3 >> 8),cVar5);
        if (cVar5 == '\0') {
          uVar14 = FUN_1920_0da6();
          return uVar14;
        }
      }
    }
    else {
      in_AX = uVar4 + uVar6;
      uVar9 = uVar8 + unaff_SI + (uint)CARRY2(uVar4,uVar6);
      uVar10 = (uint)(CARRY2(uVar8,unaff_SI) || CARRY2(uVar8 + unaff_SI,(uint)CARRY2(uVar4,uVar6)));
      uVar4 = uVar7 + uVar11;
      in_DX = uVar4 + uVar10;
      if (CARRY2(uVar7,uVar11) || CARRY2(uVar4,uVar10)) {
        uVar8 = in_DX & 1;
        in_DX = in_DX >> 1 | (uint)(CARRY2(uVar7,uVar11) || CARRY2(uVar4,uVar10)) << 0xf;
        uVar10 = uVar9 & 1;
        uVar9 = uVar9 >> 1 | (uint)(uVar8 != 0) << 0xf;
        in_AX = in_AX >> 1 | (uint)(uVar10 != 0) << 0xf;
        cVar5 = (char)uVar3 + '\x01';
        uVar3 = CONCAT11((char)(uVar3 >> 8),cVar5);
        if (cVar5 == '\0') goto LAB_1920_0c72;
      }
    }
    bVar12 = 0xff7f < in_AX;
    in_AX = in_AX + 0x80;
    uVar10 = (uint)CARRY2(uVar9,(uint)bVar12);
    bVar12 = CARRY2(in_DX,uVar10);
    in_DX = in_DX + uVar10;
    if (bVar12) {
      in_DX = in_DX >> 1 | 0x8000;
      cVar5 = (char)uVar3 + '\x01';
      uVar3 = CONCAT11((char)(uVar3 >> 8),cVar5);
      if (cVar5 == '\0') {
LAB_1920_0c72:
        return CONCAT22(in_DX,in_AX);
      }
    }
    in_AX = CONCAT11((char)(in_AX >> 8),(char)uVar3);
    in_DX = CONCAT11((byte)((in_DX & 0x7fff) >> 8) | (byte)(uVar3 >> 8),(char)(in_DX & 0x7fff));
  }
LAB_1920_0c64:
  return CONCAT22(in_DX,in_AX);
}



// ================================================
// Function: FUN_1920_0be5 at 1920:0be5
// ================================================

undefined4 __cdecl16near FUN_1920_0be5(void)

{
  uint uVar1;
  byte bVar2;
  uint in_AX;
  uint uVar3;
  uint uVar4;
  char cVar5;
  uint in_CX;
  uint uVar6;
  uint in_DX;
  uint uVar7;
  uint in_BX;
  uint uVar8;
  uint uVar9;
  uint unaff_SI;
  uint unaff_DI;
  uint uVar10;
  bool bVar11;
  bool bVar12;
  undefined4 uVar13;
  
  if ((byte)in_CX != 0) {
    if ((byte)in_AX == 0) {
LAB_1920_0c65:
      return CONCAT22(unaff_DI,in_CX);
    }
    uVar4 = in_AX;
    uVar7 = in_DX;
    uVar8 = in_BX;
    if ((byte)in_CX < (byte)in_AX) {
      uVar4 = in_CX;
      in_CX = in_AX;
      uVar7 = unaff_DI;
      uVar8 = unaff_SI;
      unaff_SI = in_BX;
      unaff_DI = in_DX;
    }
    bVar2 = -((char)uVar4 - (char)in_CX);
    if (0x27 < bVar2) goto LAB_1920_0c65;
    uVar3 = CONCAT11((char)(uVar7 >> 8),(char)in_CX) & 0x80ff;
    uVar4 = uVar4 & 0xff00;
    uVar7 = uVar7 | 0x8000;
    uVar10 = unaff_DI | 0x8000;
    for (; 7 < bVar2; bVar2 = bVar2 - 8) {
      uVar4 = CONCAT11((char)uVar8,(char)(uVar4 >> 8));
      uVar8 = CONCAT11((char)uVar7,(char)(uVar8 >> 8));
      uVar7 = uVar7 >> 8;
    }
    uVar6 = CONCAT11((char)(in_CX >> 8),bVar2);
    while (bVar2 != 0) {
      uVar1 = uVar7 & 1;
      uVar7 = uVar7 >> 1;
      uVar9 = uVar8 & 1;
      uVar8 = uVar8 >> 1 | (uint)(uVar1 != 0) << 0xf;
      uVar4 = uVar4 >> 1 | (uint)(uVar9 != 0) << 0xf;
      bVar2 = (char)uVar6 - 1;
      uVar6 = CONCAT11((char)(uVar6 >> 8),bVar2);
    }
    if ((int)(uVar3 ^ unaff_DI) < 0) {
      in_AX = uVar4 - uVar6;
      uVar9 = (uVar8 - unaff_SI) - (uint)(uVar4 < uVar6);
      uVar4 = (uint)(uVar8 < unaff_SI || uVar8 - unaff_SI < (uint)(uVar4 < uVar6));
      in_DX = (uVar7 - uVar10) - uVar4;
      if (uVar7 < uVar10 || uVar7 - uVar10 < uVar4) {
        uVar7 = ~uVar9;
        bVar11 = in_AX == 0;
        in_AX = -in_AX;
        uVar9 = uVar7 + bVar11;
        in_DX = ~in_DX + (uint)CARRY2(uVar7,(uint)bVar11);
        uVar3 = uVar3 ^ 0x8000;
      }
      if ((in_DX == 0 && uVar9 == 0) && in_AX == 0) goto LAB_1920_0c64;
      while (-1 < (int)in_DX) {
        bVar11 = (int)in_AX < 0;
        in_AX = in_AX << 1;
        bVar12 = (int)uVar9 < 0;
        uVar9 = uVar9 << 1 | (uint)bVar11;
        in_DX = in_DX << 1 | (uint)bVar12;
        cVar5 = (char)uVar3 + -1;
        uVar3 = CONCAT11((char)(uVar3 >> 8),cVar5);
        if (cVar5 == '\0') {
          uVar13 = FUN_1920_0da6();
          return uVar13;
        }
      }
    }
    else {
      in_AX = uVar4 + uVar6;
      uVar9 = uVar8 + unaff_SI + (uint)CARRY2(uVar4,uVar6);
      uVar4 = (uint)(CARRY2(uVar8,unaff_SI) || CARRY2(uVar8 + unaff_SI,(uint)CARRY2(uVar4,uVar6)));
      uVar8 = uVar7 + uVar10;
      in_DX = uVar8 + uVar4;
      if (CARRY2(uVar7,uVar10) || CARRY2(uVar8,uVar4)) {
        uVar6 = in_DX & 1;
        in_DX = in_DX >> 1 | (uint)(CARRY2(uVar7,uVar10) || CARRY2(uVar8,uVar4)) << 0xf;
        uVar7 = uVar9 & 1;
        uVar9 = uVar9 >> 1 | (uint)(uVar6 != 0) << 0xf;
        in_AX = in_AX >> 1 | (uint)(uVar7 != 0) << 0xf;
        cVar5 = (char)uVar3 + '\x01';
        uVar3 = CONCAT11((char)(uVar3 >> 8),cVar5);
        if (cVar5 == '\0') goto LAB_1920_0c72;
      }
    }
    bVar11 = 0xff7f < in_AX;
    in_AX = in_AX + 0x80;
    uVar7 = (uint)CARRY2(uVar9,(uint)bVar11);
    bVar11 = CARRY2(in_DX,uVar7);
    in_DX = in_DX + uVar7;
    if (bVar11) {
      in_DX = in_DX >> 1 | 0x8000;
      cVar5 = (char)uVar3 + '\x01';
      uVar3 = CONCAT11((char)(uVar3 >> 8),cVar5);
      if (cVar5 == '\0') {
LAB_1920_0c72:
        return CONCAT22(in_DX,in_AX);
      }
    }
    in_AX = CONCAT11((char)(in_AX >> 8),(char)uVar3);
    in_DX = CONCAT11((byte)((in_DX & 0x7fff) >> 8) | (byte)(uVar3 >> 8),(char)(in_DX & 0x7fff));
  }
LAB_1920_0c64:
  return CONCAT22(in_DX,in_AX);
}



// ================================================
// Function: FUN_1920_0ca8 at 1920:0ca8
// ================================================

undefined4 __cdecl16near FUN_1920_0ca8(void)

{
  long lVar1;
  long lVar2;
  ulong uVar3;
  long lVar4;
  byte bVar5;
  undefined2 in_AX;
  byte bVar7;
  uint uVar6;
  byte bVar8;
  uint in_CX;
  int iVar9;
  uint in_DX;
  uint uVar10;
  uint uVar11;
  uint uVar12;
  uint uVar13;
  uint uVar14;
  uint in_BX;
  uint uVar15;
  uint uVar16;
  uint uVar17;
  uint uVar18;
  uint uVar19;
  uint uVar20;
  uint unaff_SI;
  uint unaff_DI;
  uint uVar21;
  uint uVar22;
  uint uVar23;
  bool bVar24;
  undefined4 uVar25;
  
  bVar5 = (byte)in_AX;
  bVar7 = (byte)((uint)in_AX >> 8);
  if ((bVar5 == 0) || (bVar8 = (byte)in_CX, bVar8 == 0)) {
    uVar25 = FUN_1920_0da6();
    return uVar25;
  }
  iVar9 = CONCAT11((((byte)(in_DX >> 8) ^ (byte)(unaff_DI >> 8)) & 0x80) + CARRY1(bVar5,bVar8),
                   bVar5 + bVar8);
  uVar20 = in_DX | 0x8000;
  uVar21 = unaff_DI | 0x8000;
  if (((bVar7 == 0) &&
      (uVar6 = in_CX & 0xff00, uVar14 = uVar20, uVar10 = unaff_SI, uVar22 = uVar21, in_BX == 0)) ||
     ((bVar5 = (byte)(in_CX >> 8), bVar5 == 0 &&
      (uVar6 = (uint)bVar7 << 8, uVar14 = uVar21, uVar10 = in_BX, uVar22 = uVar20, unaff_SI == 0))))
  {
    uVar3 = (ulong)uVar14 * (ulong)uVar10 + (ulong)((uVar14 >> 8) * (uVar6 >> 8));
    uVar15 = (uint)uVar3;
    lVar1 = (ulong)uVar14 * (ulong)uVar22 + (uVar3 >> 0x10);
  }
  else {
    lVar1 = (ulong)bVar7 * 0x100 * (ulong)unaff_SI;
    lVar4 = lVar1 + (ulong)((uint)bVar7 * (uint)bVar5);
    uVar22 = (uint)((ulong)lVar4 >> 0x10);
    lVar2 = (ulong)in_BX * (ulong)bVar5 * 0x100;
    uVar10 = (uint)((ulong)lVar2 >> 0x10);
    uVar6 = (uint)CARRY2((uint)lVar4,(uint)lVar2);
    uVar14 = uVar22 + uVar10;
    uVar15 = uVar14 + uVar6;
    uVar16 = (uint)CARRY2((uint)((ulong)lVar1 >> 0x10),
                          (uint)CARRY2((uint)bVar7 * (uint)bVar5,(uint)lVar1)) +
             (uint)(CARRY2(uVar22,uVar10) || CARRY2(uVar14,uVar6));
    lVar1 = (ulong)bVar7 * 0x100 * (ulong)uVar21;
    uVar11 = (uint)((ulong)lVar1 >> 0x10);
    uVar6 = (uint)lVar1;
    uVar23 = uVar15 + uVar6;
    uVar6 = (uint)CARRY2(uVar15,uVar6);
    uVar22 = uVar16 + uVar11;
    uVar17 = uVar22 + uVar6;
    uVar12 = (uint)((ulong)in_BX * (ulong)unaff_SI >> 0x10);
    uVar10 = (uint)((ulong)in_BX * (ulong)unaff_SI);
    uVar14 = (uint)CARRY2(uVar23,uVar10);
    uVar15 = uVar17 + uVar12;
    uVar18 = uVar15 + uVar14;
    lVar1 = (ulong)uVar20 * (ulong)bVar5 * 0x100;
    uVar13 = (uint)((ulong)lVar1 >> 0x10);
    uVar10 = (uint)CARRY2(uVar23 + uVar10,(uint)lVar1);
    uVar23 = uVar18 + uVar13;
    uVar19 = uVar23 + uVar10;
    uVar12 = (uint)(CARRY2(uVar16,uVar11) || CARRY2(uVar22,uVar6)) +
             (uint)(CARRY2(uVar17,uVar12) || CARRY2(uVar15,uVar14)) +
             (uint)(CARRY2(uVar18,uVar13) || CARRY2(uVar23,uVar10));
    uVar23 = (uint)((ulong)in_BX * (ulong)uVar21 >> 0x10);
    uVar6 = (uint)((ulong)in_BX * (ulong)uVar21);
    uVar22 = uVar19 + uVar6;
    uVar6 = (uint)CARRY2(uVar19,uVar6);
    uVar10 = uVar12 + uVar23;
    uVar13 = uVar10 + uVar6;
    uVar11 = (uint)((ulong)uVar20 * (ulong)unaff_SI >> 0x10);
    uVar14 = (uint)((ulong)uVar20 * (ulong)unaff_SI);
    uVar15 = uVar22 + uVar14;
    uVar14 = (uint)CARRY2(uVar22,uVar14);
    uVar22 = uVar13 + uVar11;
    lVar1 = (ulong)uVar20 * (ulong)uVar21 +
            CONCAT22((uint)(CARRY2(uVar12,uVar23) || CARRY2(uVar10,uVar6)) +
                     (uint)(CARRY2(uVar13,uVar11) || CARRY2(uVar22,uVar14)),uVar22 + uVar14);
  }
  if (-1 < lVar1) {
    bVar24 = (int)uVar15 < 0;
    uVar15 = uVar15 << 1;
    lVar1 = CONCAT22((int)((ulong)lVar1 >> 0x10) << 1 | (uint)((int)lVar1 < 0),
                     (int)lVar1 << 1 | (uint)bVar24);
    iVar9 = iVar9 + -1;
  }
  uVar6 = (uint)((ulong)lVar1 >> 0x10);
  uVar21 = iVar9 + 0x7f7f;
  uVar20 = (uint)CARRY2((uint)lVar1,(uint)(0xff7f < uVar15));
  uVar14 = uVar6 + uVar20;
  if (CARRY2(uVar6,uVar20)) {
    uVar14 = uVar14 >> 1 | 0x8000;
    uVar21 = iVar9 + 0x7f80;
  }
  if ((uVar21 & 0x4000) != 0) {
    return 0;
  }
  return CONCAT22(CONCAT11((byte)(uVar14 >> 8) ^ (byte)(uVar21 + 1 >> 8),(char)uVar14),
                  CONCAT11((char)(uVar15 + 0x80 >> 8),(char)(uVar21 + 1)));
}



// ================================================
// Function: FUN_1920_0da6 at 1920:0da6
// ================================================

void __cdecl16near FUN_1920_0da6(void)

{
  return;
}



// ================================================
// Function: FUN_1920_0dad at 1920:0dad
// ================================================

undefined4 __cdecl16near FUN_1920_0dad(void)

{
  byte bVar1;
  undefined1 uVar2;
  char cVar3;
  undefined2 in_AX;
  int iVar4;
  byte bVar5;
  undefined2 in_CX;
  byte bVar7;
  uint uVar6;
  uint in_DX;
  char cVar9;
  uint uVar8;
  uint in_BX;
  uint uVar10;
  uint unaff_BP;
  uint uVar11;
  uint unaff_SI;
  uint unaff_DI;
  uint uVar12;
  bool bVar13;
  bool bVar14;
  int in_stack_00000000;
  
  bVar1 = (byte)in_AX;
  if (bVar1 == 0) {
    return 0;
  }
  uVar12 = unaff_DI | 0x8000;
  uVar11 = in_DX | 0x8000;
  cVar9 = (((byte)(in_DX >> 8) ^ (byte)(unaff_DI >> 8)) & 0x80) - (bVar1 < (byte)in_CX);
  iVar4 = CONCAT11((char)((uint)in_AX >> 8),2);
  uVar8 = 1;
  do {
    bVar13 = uVar11 < uVar12;
    bVar7 = (byte)((uint)in_CX >> 8);
    bVar5 = (byte)((uint)iVar4 >> 8);
    if ((uVar11 == uVar12) && (bVar13 = in_BX < unaff_SI, in_BX == unaff_SI)) {
      bVar13 = bVar5 < bVar7;
    }
    if (!bVar13) {
      iVar4 = CONCAT11(bVar5 - bVar7,(char)iVar4);
      bVar13 = in_BX < unaff_SI;
      uVar10 = in_BX - unaff_SI;
      in_BX = uVar10 - (bVar5 < bVar7);
      uVar10 = (uint)(bVar13 || uVar10 < (bVar5 < bVar7));
      bVar13 = uVar11 < uVar12 || uVar11 - uVar12 < uVar10;
      uVar11 = (uVar11 - uVar12) - uVar10;
    }
    while( true ) {
      bVar14 = (int)uVar8 < 0;
      uVar8 = uVar8 << 1 | (uint)bVar13;
      if (bVar14) {
        cVar3 = (char)iVar4 + -1;
        iVar4 = CONCAT11((char)((uint)iVar4 >> 8),cVar3);
        if (cVar3 < '\0') {
          uVar11 = ~(uVar8 << 6);
          uVar10 = ~CONCAT11(cVar9,bVar1 - (byte)in_CX);
          uVar12 = unaff_BP ^ 0xffff;
          if (-1 < (int)uVar12) {
            uVar11 = uVar11 << 1;
            uVar10 = uVar10 << 1 | (uint)(-1 < (int)(uVar8 << 6));
            uVar12 = uVar12 << 1 | (uint)(-1 < cVar9);
            in_stack_00000000 = in_stack_00000000 + -1;
          }
          uVar6 = in_stack_00000000 + 0x8080;
          uVar8 = (uint)CARRY2(uVar10,(uint)(0xff7f < uVar11));
          uVar10 = uVar12 + uVar8;
          if (CARRY2(uVar12,uVar8)) {
            uVar10 = uVar10 >> 1 | 0x8000;
            uVar6 = in_stack_00000000 + 0x8081;
          }
          if ((uVar6 & 0x4000) != 0) {
            return 0;
          }
          return CONCAT22(CONCAT11((byte)(uVar10 >> 8) ^ (byte)(uVar6 + 1 >> 8),(char)uVar10),
                          CONCAT11((char)(uVar11 + 0x80 >> 8),(char)(uVar6 + 1)));
        }
        uVar8 = 1;
        if (cVar3 == '\0') {
          uVar8 = 0x40;
        }
      }
      bVar13 = iVar4 < 0;
      uVar2 = (undefined1)iVar4;
      bVar5 = (char)((uint)iVar4 >> 8) * '\x02';
      iVar4 = CONCAT11(bVar5,uVar2);
      bVar14 = (int)in_BX < 0;
      in_BX = in_BX << 1 | (uint)bVar13;
      bVar13 = -1 < (int)uVar11;
      uVar11 = uVar11 << 1 | (uint)bVar14;
      if (bVar13) break;
      iVar4 = CONCAT11(bVar5 - bVar7,uVar2);
      bVar13 = in_BX < unaff_SI;
      uVar10 = in_BX - unaff_SI;
      in_BX = uVar10 - (bVar5 < bVar7);
      uVar11 = (uVar11 - uVar12) - (uint)(bVar13 || uVar10 < (bVar5 < bVar7));
      bVar13 = false;
    }
  } while( true );
}



// ================================================
// Function: FUN_1920_0e24 at 1920:0e24
// ================================================

void __cdecl16near FUN_1920_0e24(void)

{
  uint in_DX;
  uint unaff_DI;
  bool bVar1;
  
  if ((int)(in_DX ^ unaff_DI) < 0) {
    return;
  }
  bVar1 = (in_DX & 0x8000) == 0;
  if ((!bVar1) && (FUN_1920_0e3b(), !bVar1)) {
    return;
  }
  return;
}



// ================================================
// Function: FUN_1920_0e3b at 1920:0e3b
// ================================================

void __cdecl16near FUN_1920_0e3b(void)

{
  return;
}



// ================================================
// Function: FUN_1920_0e4e at 1920:0e4e
// ================================================

void __cdecl16near FUN_1920_0e4e(void)

{
  uint in_AX;
  uint uVar1;
  uint in_DX;
  
  if (in_AX != 0 || in_DX != 0) {
    if ((int)in_DX < 0) {
      uVar1 = ~in_AX;
      in_AX = uVar1 + 1;
      in_DX = ~in_DX + (uint)(0xfffe < uVar1);
    }
    uVar1 = in_AX;
    if ((in_DX == 0) && (uVar1 = 0, in_DX = in_AX, (char)(in_AX >> 8) == '\0')) {
      in_DX = in_AX << 8;
    }
    while (-1 < (int)in_DX) {
      in_DX = in_DX * 2 + (uint)CARRY2(uVar1,uVar1);
      uVar1 = uVar1 * 2;
    }
  }
  return;
}



// ================================================
// Function: FUN_1920_0e8d at 1920:0e8d
// ================================================

undefined4 __cdecl16near FUN_1920_0e8d(void)

{
  uint uVar1;
  char cVar2;
  byte in_AL;
  uint uVar3;
  byte bVar4;
  char in_CH;
  uint in_DX;
  uint uVar5;
  uint in_BX;
  bool bVar6;
  
  cVar2 = -in_AL;
  uVar5 = in_DX;
  if (in_AL < 0xa0) {
    if (0x1f < (byte)(cVar2 + 0x9fU)) {
      return 0;
    }
    bVar4 = cVar2 + 0xa0;
    uVar5 = in_DX | 0x8000;
    if (0x10 < bVar4) {
      uVar5 = 0;
      bVar4 = cVar2 + 0x90;
      in_BX = in_DX | 0x8000;
    }
    if (8 < bVar4) {
      in_BX = CONCAT11((char)uVar5,(char)(in_BX >> 8));
      uVar5 = uVar5 >> 8;
      bVar4 = bVar4 - 8;
    }
    do {
      uVar1 = uVar5 & 1;
      uVar5 = uVar5 >> 1;
      uVar3 = in_BX & 1;
      in_BX = in_BX >> 1 | (uint)(uVar1 != 0) << 0xf;
      bVar4 = bVar4 - 1;
    } while (bVar4 != 0);
    if ((uVar3 != 0) && (in_CH != '\0')) {
      bVar6 = 0xfffe < in_BX;
      in_BX = in_BX + 1;
      uVar5 = uVar5 + bVar6;
      if ((int)uVar5 < 0) goto LAB_1920_0ee2;
    }
    if ((int)in_DX < 0) {
      uVar3 = ~in_BX;
      in_BX = uVar3 + 1;
      uVar5 = ~uVar5 + (uint)(0xfffe < uVar3);
    }
    return CONCAT22(uVar5,in_BX);
  }
LAB_1920_0ee2:
  return CONCAT22(uVar5,in_BX);
}



// ================================================
// Function: FUN_1920_0ee9 at 1920:0ee9
// ================================================

void __cdecl16far FUN_1920_0ee9(void)

{
  undefined1 in_CF;
  
  FUN_1920_0be5();
  if (!(bool)in_CF) {
    return;
  }
  FUN_1920_00e2();
  return;
}



// ================================================
// Function: FUN_1920_0efb at 1920:0efb
// ================================================

void __cdecl16far FUN_1920_0efb(void)

{
  undefined1 in_CF;
  
  FUN_1920_0ca8();
  if (!(bool)in_CF) {
    return;
  }
  FUN_1920_00e2();
  return;
}



// ================================================
// Function: FUN_1920_0f01 at 1920:0f01
// ================================================

void __cdecl16far FUN_1920_0f01(void)

{
  char in_CL;
  bool bVar1;
  
  bVar1 = false;
  if (in_CL == '\0') {
    FUN_1920_00e2();
    return;
  }
  FUN_1920_0dad();
  if (!bVar1) {
    return;
  }
  FUN_1920_00e2();
  return;
}



// ================================================
// Function: FUN_1920_0f0b at 1920:0f0b
// ================================================

void __cdecl16far FUN_1920_0f0b(void)

{
  FUN_1920_0e24();
  return;
}



// ================================================
// Function: FUN_1920_0f0f at 1920:0f0f
// ================================================

void __cdecl16far FUN_1920_0f0f(void)

{
  FUN_1920_0e4e();
  return;
}



// ================================================
// Function: FUN_1920_0f13 at 1920:0f13
// ================================================

void __cdecl16far FUN_1920_0f13(void)

{
  undefined1 in_CF;
  
  FUN_1920_0e8d();
  if (!(bool)in_CF) {
    return;
  }
  FUN_1920_00e2();
  return;
}



// ================================================
// Function: FUN_1920_0f1b at 1920:0f1b
// ================================================

void __cdecl16far FUN_1920_0f1b(void)

{
  undefined1 in_CF;
  
  FUN_1920_0e8d();
  if (!(bool)in_CF) {
    return;
  }
  FUN_1920_00e2();
  return;
}



// ================================================
// Function: FUN_1920_0f35 at 1920:0f35
// ================================================

void __cdecl16near FUN_1920_0f35(void)

{
  FUN_1920_0be5();
  return;
}



// ================================================
// Function: FUN_1920_0f3f at 1920:0f3f
// ================================================

void __cdecl16near FUN_1920_0f3f(void)

{
  FUN_1920_0be1();
  return;
}



// ================================================
// Function: FUN_1920_0f49 at 1920:0f49
// ================================================

void __cdecl16near FUN_1920_0f49(void)

{
  FUN_1920_0ca8();
  return;
}



// ================================================
// Function: FUN_1920_0f53 at 1920:0f53
// ================================================

void __cdecl16near FUN_1920_0f53(void)

{
  FUN_1920_0dad();
  return;
}



// ================================================
// Function: FUN_1920_0f5d at 1920:0f5d
// ================================================

uint __cdecl16far FUN_1920_0f5d(void)

{
  uint uVar1;
  uint uVar2;
  byte bVar3;
  uint in_AX;
  uint uVar4;
  byte bVar5;
  uint uVar6;
  undefined1 uVar8;
  uint uVar7;
  
  bVar3 = (byte)in_AX;
  if (bVar3 < 0xa8) {
    uVar4 = in_AX & 0xff;
    bVar5 = bVar3 + 0x80;
    uVar1 = 0;
    uVar7 = 0;
    if (bVar3 < 0x80 || bVar5 == 0) {
      return 0;
    }
    while( true ) {
      uVar6 = uVar1;
      uVar8 = (undefined1)(uVar7 >> 8);
      if (bVar5 < 0x10) break;
      uVar4 = CONCAT11(uVar8,(char)uVar4);
      bVar5 = bVar5 - 0x10;
      uVar1 = 0xffff;
      uVar7 = uVar6;
    }
    if (7 < bVar5) {
      uVar4 = CONCAT11((char)uVar7,(char)uVar4);
      uVar7 = CONCAT11((char)uVar6,uVar8);
      uVar6 = CONCAT11(0xff,(char)(uVar6 >> 8));
      bVar5 = bVar5 - 8;
    }
    while( true ) {
      bVar3 = (byte)(uVar4 >> 8);
      if (bVar5 == 0) break;
      uVar1 = uVar6 & 1;
      uVar6 = uVar6 >> 1 | 0x8000;
      uVar2 = uVar7 & 1;
      uVar7 = uVar7 >> 1 | (uint)(uVar1 != 0) << 0xf;
      uVar4 = CONCAT11(bVar3 >> 1 | (uVar2 != 0) << 7,(char)uVar4);
      bVar5 = bVar5 - 1;
    }
    in_AX = CONCAT11(bVar3 & (byte)(in_AX >> 8),(char)uVar4);
  }
  return in_AX;
}



// ================================================
// Function: FUN_1920_0fae at 1920:0fae
// ================================================

void __cdecl16far FUN_1920_0fae(void)

{
  FUN_1920_0f5d();
  FUN_1920_0be1();
  return;
}



// ================================================
// Function: FUN_1920_0fc2 at 1920:0fc2
// ================================================

uint __cdecl16far FUN_1920_0fc2(void)

{
  char cVar1;
  byte bVar2;
  uint in_AX;
  uint uVar3;
  uint in_DX;
  undefined2 in_BX;
  undefined2 uVar4;
  undefined4 uVar5;
  
  cVar1 = (char)in_AX;
  if (cVar1 != '\0') {
    if ((in_DX & 0x8000) != 0) {
      uVar3 = FUN_1920_00e2();
      return uVar3;
    }
    do {
      uVar4 = in_BX;
      FUN_1920_0f53();
      uVar5 = FUN_1920_0f35();
      in_AX = (uint)(byte)((char)uVar5 - 1);
      bVar2 = FUN_1920_0be1(in_AX,uVar4,(int)((ulong)uVar5 >> 0x10));
    } while ((byte)(((char)(cVar1 + -0x80) >> 1) + 0x6cU) <= bVar2);
  }
  return in_AX;
}



// ================================================
// Function: FUN_1920_1021 at 1920:1021
// ================================================

void FUN_1920_1021(void)

{
  byte bVar1;
  undefined2 uVar2;
  uint extraout_DX;
  uint uVar3;
  uint extraout_DX_00;
  bool bVar4;
  undefined1 uVar5;
  
  bVar1 = FUN_1920_0be5();
  uVar3 = extraout_DX;
  if (bVar1 != 0) {
    uVar3 = extraout_DX ^ 0x8000;
  }
  if (0x6b < bVar1) {
    uVar2 = 0x2183;
    bVar4 = false;
    FUN_1920_0e24();
    if (!bVar4) {
      FUN_1920_0f53();
      FUN_1920_0fae(uVar2,0xdaa2,0x490f);
      FUN_1920_0f49();
      uVar3 = extraout_DX_00;
    }
    uVar5 = 0;
    if ((uVar3 & 0x8000) != 0) {
      FUN_1920_0f35();
    }
    FUN_1920_0e24();
    if (!(bool)uVar5) {
      FUN_1920_0f3f();
    }
    bVar1 = FUN_1920_0e24();
    if (!(bool)uVar5) {
      bVar1 = FUN_1920_0be5();
    }
    if (0x6b < bVar1) {
      FUN_1920_1340();
    }
  }
  return;
}



// ================================================
// Function: FUN_1920_1034 at 1920:1034
// ================================================

void __cdecl16far FUN_1920_1034(void)

{
  byte in_AL;
  byte bVar1;
  undefined2 uVar2;
  uint in_DX;
  uint extraout_DX;
  bool bVar3;
  undefined1 uVar4;
  
  if (0x6b < in_AL) {
    uVar2 = 0x2183;
    bVar3 = false;
    FUN_1920_0e24();
    if (!bVar3) {
      FUN_1920_0f53();
      FUN_1920_0fae(uVar2,0xdaa2,0x490f);
      FUN_1920_0f49();
      in_DX = extraout_DX;
    }
    uVar4 = 0;
    if ((in_DX & 0x8000) != 0) {
      FUN_1920_0f35();
    }
    FUN_1920_0e24();
    if (!(bool)uVar4) {
      FUN_1920_0f3f();
    }
    bVar1 = FUN_1920_0e24();
    if (!(bool)uVar4) {
      bVar1 = FUN_1920_0be5();
    }
    if (0x6b < bVar1) {
      FUN_1920_1340();
    }
  }
  return;
}



// ================================================
// Function: FUN_1920_1340 at 1920:1340
// ================================================

void FUN_1920_1340(void)

{
  FUN_1920_0ca8();
  FUN_1920_1359();
  FUN_1920_0ca8();
  return;
}



// ================================================
// Function: FUN_1920_1359 at 1920:1359
// ================================================

void __cdecl16near FUN_1920_1359(void)

{
  int in_CX;
  int unaff_DI;
  undefined2 uStack_a;
  
  uStack_a = in_CX;
  while( true ) {
    FUN_1920_0ca8();
    unaff_DI = unaff_DI + 6;
    uStack_a = uStack_a + -1;
    if (uStack_a == 0) break;
    FUN_1920_0be5(unaff_DI);
  }
  FUN_1920_0be5();
  return;
}



// ================================================
// Function: FUN_1920_13a8 at 1920:13a8
// ================================================

uint __stdcall16far FUN_1920_13a8(uint param_1)

{
  uint uVar1;
  uint extraout_DX;
  
  FUN_1920_13f7();
  uVar1 = 0;
  if (param_1 != 0) {
    uVar1 = extraout_DX % param_1;
  }
  return uVar1;
}



// ================================================
// Function: FUN_1920_13f7 at 1920:13f7
// ================================================

void __cdecl16near FUN_1920_13f7(void)

{
  int iVar1;
  long lVar2;
  uint uVar3;
  char cVar4;
  int iVar5;
  undefined2 unaff_DS;
  
  iVar1 = *(int *)0x1b00;
  lVar2 = (ulong)*(uint *)0x1afe * (ulong)uRam0001142d;
  uVar3 = (uint)lVar2;
  iVar5 = *(uint *)0x1afe << 3;
  cVar4 = (char)iVar5;
  iVar5 = (int)((ulong)lVar2 >> 0x10) + CONCAT11((char)((uint)iVar5 >> 8) + cVar4,cVar4) + iVar1 * 5
  ;
  *(int *)0x1afe = uVar3 + 1;
  *(int *)0x1b00 =
       CONCAT11((char)((uint)iVar5 >> 8) + (char)iVar1 * '\x04' + (char)(iVar1 << 7),(char)iVar5) +
       (uint)(0xfffe < uVar3);
  return;
}



// ================================================
// Function: FUN_1920_142f at 1920:142f
// ================================================

void __cdecl16far FUN_1920_142f(void)

{
  code *pcVar1;
  undefined2 in_CX;
  undefined2 extraout_DX;
  undefined2 unaff_DS;
  
  pcVar1 = (code *)swi(0x21);
  (*pcVar1)();
  *(undefined2 *)0x1afe = in_CX;
  *(undefined2 *)0x1b00 = extraout_DX;
  return;
}



// ================================================
// Function: FUN_1920_143c at 1920:143c
// ================================================

void __cdecl16near FUN_1920_143c(void)

{
  char *pcVar1;
  uint uVar2;
  char cVar3;
  uint in_AX;
  char cVar4;
  uint in_DX;
  uint uVar5;
  uint *puVar6;
  char *unaff_DI;
  undefined2 unaff_ES;
  bool bVar7;
  
  if ((int)in_DX < 0) {
    uVar5 = ~in_AX;
    in_AX = uVar5 + 1;
    in_DX = ~in_DX + (uint)(0xfffe < uVar5);
    pcVar1 = unaff_DI;
    unaff_DI = unaff_DI + 1;
    *pcVar1 = '-';
  }
  puVar6 = (uint *)0x148e;
  cVar4 = '\t';
  do {
    if ((puVar6[1] <= in_DX) && ((puVar6[1] < in_DX || (*puVar6 <= in_AX)))) break;
    puVar6 = puVar6 + 2;
    cVar4 = cVar4 + -1;
  } while (cVar4 != '\0');
  cVar4 = cVar4 + '\x01';
  do {
    cVar3 = '/';
    do {
      cVar3 = cVar3 + '\x01';
      bVar7 = in_AX < *puVar6;
      in_AX = in_AX - *puVar6;
      uVar5 = (uint)bVar7;
      bVar7 = puVar6[1] <= in_DX;
      uVar2 = in_DX - puVar6[1];
      in_DX = uVar2 - uVar5;
    } while (bVar7 && uVar5 <= uVar2);
    bVar7 = CARRY2(in_AX,*puVar6);
    in_AX = in_AX + *puVar6;
    in_DX = in_DX + puVar6[1] + (uint)bVar7;
    puVar6 = puVar6 + 2;
    pcVar1 = unaff_DI;
    unaff_DI = unaff_DI + 1;
    *pcVar1 = cVar3;
    cVar4 = cVar4 + -1;
  } while (cVar4 != '\0');
  return;
}



// ================================================
// Function: FUN_1920_14b6 at 1920:14b6
// ================================================

undefined4 __cdecl16near FUN_1920_14b6(void)

{
  bool bVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  int in_CX;
  uint uVar7;
  uint uVar8;
  byte bVar9;
  byte bVar10;
  char cVar11;
  byte *unaff_DI;
  undefined2 unaff_ES;
  bool bVar12;
  bool bVar13;
  
  uVar2 = 0;
  uVar7 = 0;
  bVar1 = false;
  if (in_CX == 0) goto LAB_1920_1517;
  if (*unaff_DI == 0x2b) {
LAB_1920_14cb:
    unaff_DI = unaff_DI + 1;
    in_CX = in_CX + -1;
    if (in_CX == 0) goto LAB_1920_1517;
  }
  else if (*unaff_DI == 0x2d) {
    bVar1 = true;
    goto LAB_1920_14cb;
  }
  if (*unaff_DI == 0x24) {
    iVar4 = in_CX + -1;
    if (iVar4 == 0) {
LAB_1920_1517:
      return CONCAT22(uVar7,uVar2);
    }
    do {
      unaff_DI = unaff_DI + 1;
      bVar9 = *unaff_DI;
      if (0x60 < bVar9) {
        bVar9 = bVar9 - 0x20;
      }
      bVar10 = bVar9 - 0x30;
      if ((byte)(bVar9 - 0x3a) < 0xf6) {
        if ((byte)(bVar9 + 0xb9) < 0xfa) break;
        bVar10 = bVar9 - 0x37;
      }
      cVar11 = '\x04';
      do {
        bVar12 = (int)uVar2 < 0;
        uVar2 = uVar2 << 1;
        bVar13 = (int)uVar7 < 0;
        uVar7 = uVar7 << 1 | (uint)bVar12;
        if (bVar13) goto LAB_1920_1517;
        cVar11 = cVar11 + -1;
      } while (cVar11 != '\0');
      uVar2 = CONCAT11((char)(uVar2 >> 8),(byte)uVar2 | bVar10);
      iVar4 = iVar4 + -1;
    } while (iVar4 != 0);
  }
  else {
    do {
      uVar6 = (uint)(byte)(*unaff_DI - 0x30);
      if ((byte)(*unaff_DI - 0x3a) < 0xf6) break;
      if ((uVar7 & 0xf000) != 0) goto LAB_1920_1517;
      uVar3 = uVar2 * 2;
      uVar8 = uVar7 << 1 | (uint)((int)uVar2 < 0);
      iVar4 = uVar2 << 2;
      uVar5 = uVar2 * 8;
      uVar7 = uVar2 * 10;
      uVar2 = uVar7 + uVar6;
      uVar7 = ((uVar8 << 1 | (uint)((int)uVar3 < 0)) << 1 | (uint)(iVar4 < 0)) + uVar8 +
              (uint)CARRY2(uVar5,uVar3) + (uint)CARRY2(uVar7,uVar6);
      if ((int)uVar7 < 0) goto LAB_1920_1517;
      unaff_DI = unaff_DI + 1;
      in_CX = in_CX + -1;
    } while (in_CX != 0);
  }
  if (bVar1) {
    uVar6 = ~uVar2;
    uVar2 = uVar6 + 1;
    uVar7 = ~uVar7 + (uint)(0xfffe < uVar6);
  }
  return CONCAT22(uVar7,uVar2);
}



// ================================================
// Function: FUN_1920_154e at 1920:154e
// ================================================

void __stdcall16far FUN_1920_154e(int param_1,undefined1 *param_2,int param_3)

{
  undefined1 *puVar1;
  undefined1 *puVar2;
  int in_CX;
  undefined1 *puVar3;
  undefined1 *puVar4;
  undefined2 uVar5;
  undefined2 unaff_SS;
  undefined1 local_22 [32];
  
  puVar3 = local_22;
  FUN_1920_143c();
  uVar5 = (undefined2)((ulong)param_2 >> 0x10);
  if (param_1 < param_3) {
    param_3 = param_1;
  }
  if (param_1 < in_CX) {
    in_CX = param_1;
  }
  if (param_3 < in_CX) {
    param_3 = in_CX;
  }
  puVar4 = (undefined1 *)param_2 + 1;
  *param_2 = (char)param_3;
  param_3 = param_3 - in_CX;
  if (param_3 != 0) {
    for (; param_3 != 0; param_3 = param_3 + -1) {
      puVar1 = puVar4;
      puVar4 = puVar4 + 1;
      *puVar1 = 0x20;
    }
  }
  for (; in_CX != 0; in_CX = in_CX + -1) {
    puVar2 = puVar4;
    puVar4 = puVar4 + 1;
    puVar1 = puVar3;
    puVar3 = puVar3 + 1;
    *puVar2 = *puVar1;
  }
  return;
}



// ================================================
// Function: FUN_1920_1599 at 1920:1599
// ================================================

undefined2 __stdcall16far FUN_1920_1599(int *param_1,byte *param_2)

{
  undefined2 uVar1;
  uint uVar2;
  int iVar3;
  byte *pbVar4;
  bool bVar5;
  
  bVar5 = false;
  uVar2 = (uint)*param_2;
  pbVar4 = (byte *)param_2;
  while ((pbVar4 = pbVar4 + 1, uVar2 != 0 && (bVar5 = *pbVar4 < 0x20, *pbVar4 == 0x20))) {
    uVar2 = uVar2 - 1;
  }
  uVar1 = FUN_1920_14b6();
  if ((bVar5) || (iVar3 = 0, uVar2 != 0)) {
    iVar3 = (int)pbVar4 - (int)(byte *)param_2;
    uVar1 = 0;
  }
  *param_1 = iVar3;
  return uVar1;
}



// ================================================
// Function: FUN_1920_15ca at 1920:15ca
// ================================================

void __stdcall16far FUN_1920_15ca(byte *param_1,undefined2 *param_2)

{
  byte *pbVar1;
  byte bVar2;
  int iVar3;
  uint uVar4;
  byte *pbVar5;
  byte *pbVar6;
  undefined2 uVar7;
  
  uVar7 = (undefined2)((ulong)param_2 >> 0x10);
  pbVar5 = (byte *)param_1;
  pbVar6 = (byte *)((undefined2 *)param_2 + 1);
  *param_2 = 0;
  pbVar6[0] = 0xb0;
  pbVar6[1] = 0xd7;
  for (iVar3 = 0x16; pbVar6 = pbVar6 + 2, iVar3 != 0; iVar3 = iVar3 + -1) {
    pbVar6[0] = 0;
    pbVar6[1] = 0;
  }
  bVar2 = *param_1;
  if (0x4f < bVar2) {
    bVar2 = 0x4f;
  }
  for (uVar4 = (uint)bVar2; pbVar5 = pbVar5 + 1, uVar4 != 0; uVar4 = uVar4 - 1) {
    pbVar1 = pbVar6;
    pbVar6 = pbVar6 + 1;
    *pbVar1 = *pbVar5;
  }
  *pbVar6 = 0;
  return;
}



// ================================================
// Function: FUN_1920_15f8 at 1920:15f8
// ================================================

void __stdcall16far FUN_1920_15f8(undefined2 param_1,undefined2 *param_2)

{
  code *pcVar1;
  undefined2 uVar2;
  undefined2 *puVar3;
  undefined2 uVar4;
  undefined2 unaff_DS;
  bool bVar5;
  undefined2 in_stack_00000000;
  
  uVar4 = (undefined2)((ulong)param_2 >> 0x10);
  puVar3 = (undefined2 *)param_2;
  if (puVar3[1] != -0x2850) {
    if (puVar3[1] != -0x284d) {
      *(undefined2 *)0x1afc = 0x66;
      return;
    }
    FUN_1920_1679(puVar3,uVar4);
  }
  uVar2 = 0;
  if (*(char *)(puVar3 + 0x18) != '\0') {
    bVar5 = false;
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)();
    if (bVar5) {
      *(undefined2 *)0x1afc = uVar2;
      return;
    }
  }
  puVar3[1] = 0xd7b3;
  *param_2 = uVar2;
  puVar3[2] = param_1;
  return;
}



// ================================================
// Function: FUN_1920_1601 at 1920:1601
// ================================================

void __stdcall16far FUN_1920_1601(undefined2 param_1,undefined2 *param_2)

{
  code *pcVar1;
  undefined2 uVar2;
  undefined2 *puVar3;
  undefined2 uVar4;
  undefined2 unaff_DS;
  bool bVar5;
  undefined2 in_stack_00000000;
  
  uVar4 = (undefined2)((ulong)param_2 >> 0x10);
  puVar3 = (undefined2 *)param_2;
  if (puVar3[1] != -0x2850) {
    if (puVar3[1] != -0x284d) {
      *(undefined2 *)0x1afc = 0x66;
      return;
    }
    FUN_1920_1679(puVar3,uVar4);
  }
  uVar2 = 1;
  if (*(char *)(puVar3 + 0x18) != '\0') {
    bVar5 = false;
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)();
    if (bVar5) {
      *(undefined2 *)0x1afc = uVar2;
      return;
    }
  }
  puVar3[1] = 0xd7b3;
  *param_2 = uVar2;
  puVar3[2] = param_1;
  return;
}



// ================================================
// Function: FUN_1920_1679 at 1920:1679
// ================================================

void __stdcall16far FUN_1920_1679(uint *param_1)

{
  code *pcVar1;
  undefined2 uVar2;
  undefined2 unaff_DS;
  bool bVar3;
  undefined1 in_ZF;
  
  FUN_1920_169e();
  if ((bool)in_ZF) {
    bVar3 = *param_1 < 4;
    if (4 < *param_1) {
      pcVar1 = (code *)swi(0x21);
      uVar2 = (*pcVar1)();
      if (bVar3) {
        *(undefined2 *)0x1afc = uVar2;
      }
    }
    ((uint *)param_1)[1] = 0xd7b0;
  }
  return;
}



// ================================================
// Function: FUN_1920_169e at 1920:169e
// ================================================

void __cdecl16near FUN_1920_169e(void)

{
  int unaff_DI;
  undefined2 unaff_ES;
  undefined2 unaff_DS;
  
  if (*(int *)(unaff_DI + 2) != -0x284d) {
    *(undefined2 *)0x1afc = 0x67;
  }
  return;
}



// ================================================
// Function: FUN_1920_16e3 at 1920:16e3
// ================================================

void __stdcall16far
FUN_1920_16e3(uint *param_1,uint param_2,undefined2 param_3,undefined2 param_4,undefined4 param_5)

{
  code *pcVar1;
  uint uVar2;
  undefined2 uVar3;
  undefined2 unaff_BP;
  undefined2 uVar4;
  int iVar5;
  undefined2 unaff_DS;
  bool bVar6;
  undefined1 in_ZF;
  undefined2 in_stack_00000000;
  
  uVar3 = 100;
  uVar4 = (undefined2)((ulong)param_5 >> 0x10);
  FUN_1920_169e();
  iVar5 = (int)((ulong)param_1 >> 0x10);
  if ((bool)in_ZF) {
    uVar2 = 0;
    if (param_2 != 0) {
      bVar6 = (int)((ulong)param_2 * (ulong)*(uint *)((int)param_5 + 4) >> 0x10) != 0;
      pcVar1 = (code *)swi(0x21);
      uVar2 = (*pcVar1)();
      if (bVar6) {
        *(uint *)0x1afc = uVar2;
        goto LAB_1920_1739;
      }
      uVar2 = uVar2 / *(uint *)((int)param_5 + 4);
      uVar3 = unaff_BP;
      unaff_DS = in_stack_00000000;
    }
    if (iVar5 == 0 && (uint *)param_1 == (uint *)0x0) {
      if (uVar2 != param_2) {
        *(undefined2 *)0x1afc = uVar3;
      }
    }
    else {
      *param_1 = uVar2;
    }
  }
  else {
LAB_1920_1739:
    if (iVar5 != 0 || (uint *)param_1 != (uint *)0x0) {
      *param_1 = 0;
    }
  }
  return;
}



// ================================================
// Function: FUN_1920_16ea at 1920:16ea
// ================================================

void __stdcall16far
FUN_1920_16ea(uint *param_1,uint param_2,undefined2 param_3,undefined2 param_4,undefined4 param_5)

{
  code *pcVar1;
  uint uVar2;
  undefined2 uVar3;
  undefined2 unaff_BP;
  undefined2 uVar4;
  int iVar5;
  undefined2 unaff_DS;
  bool bVar6;
  undefined1 in_ZF;
  undefined2 in_stack_00000000;
  
  uVar3 = 0x65;
  uVar4 = (undefined2)((ulong)param_5 >> 0x10);
  FUN_1920_169e();
  iVar5 = (int)((ulong)param_1 >> 0x10);
  if ((bool)in_ZF) {
    uVar2 = 0;
    if (param_2 != 0) {
      bVar6 = (int)((ulong)param_2 * (ulong)*(uint *)((int)param_5 + 4) >> 0x10) != 0;
      pcVar1 = (code *)swi(0x21);
      uVar2 = (*pcVar1)();
      if (bVar6) {
        *(uint *)0x1afc = uVar2;
        goto LAB_1920_1739;
      }
      uVar2 = uVar2 / *(uint *)((int)param_5 + 4);
      uVar3 = unaff_BP;
      unaff_DS = in_stack_00000000;
    }
    if (iVar5 == 0 && (uint *)param_1 == (uint *)0x0) {
      if (uVar2 != param_2) {
        *(undefined2 *)0x1afc = uVar3;
      }
    }
    else {
      *param_1 = uVar2;
    }
  }
  else {
LAB_1920_1739:
    if (iVar5 != 0 || (uint *)param_1 != (uint *)0x0) {
      *param_1 = 0;
    }
  }
  return;
}



// ================================================
// Function: FUN_1920_174b at 1920:174b
// ================================================

void __stdcall16far FUN_1920_174b(uint param_1,int param_2,undefined4 param_3)

{
  code *pcVar1;
  undefined2 uVar2;
  undefined2 unaff_DS;
  bool bVar3;
  undefined1 in_ZF;
  
  uVar2 = (undefined2)((ulong)param_3 >> 0x10);
  FUN_1920_169e();
  if ((bool)in_ZF) {
    bVar3 = CARRY2(param_2 * *(int *)((int)param_3 + 4),
                   (uint)((ulong)param_1 * (ulong)*(uint *)((int)param_3 + 4) >> 0x10));
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)();
    if (bVar3) {
      *(undefined2 *)0x1afc = uVar2;
    }
  }
  return;
}



// ================================================
// Function: FUN_1920_177b at 1920:177b
// ================================================

void __stdcall16far FUN_1920_177b(void)

{
  int unaff_DI;
  undefined2 unaff_ES;
  undefined1 in_CF;
  
  FUN_1920_17c2();
  if ((!(bool)in_CF) && (*(int *)(unaff_DI + 4) != 1)) {
    FUN_1920_0945();
  }
  return;
}



// ================================================
// Function: FUN_1920_17c2 at 1920:17c2
// ================================================

undefined2 __cdecl16near FUN_1920_17c2(undefined2 param_1,undefined2 param_2,undefined4 param_3)

{
  code *pcVar1;
  undefined2 uVar2;
  undefined2 unaff_DS;
  
  if (*(int *)((int)param_3 + 2) == -0x284d) {
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
    pcVar1 = (code *)swi(0x21);
    (*pcVar1)();
    pcVar1 = (code *)swi(0x21);
    uVar2 = (*pcVar1)();
    return uVar2;
  }
  *(undefined2 *)0x1afc = 0x67;
  return 0;
}



// ================================================
// Function: FUN_1920_1807 at 1920:1807
// ================================================

byte __stdcall16far FUN_1920_1807(byte param_1)

{
  if ((0x60 < param_1) && (param_1 < 0x7b)) {
    param_1 = param_1 - 0x20;
  }
  return param_1;
}



