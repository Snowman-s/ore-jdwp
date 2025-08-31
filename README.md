# Ore JDWP

オレオレ JDWP(Java Wire Debugging Protocol) 実装

自分の実験用に書いたコードです。

# 説明
起動すると、指定されたポートに接続、JVMとの接続が確立したら  
コマンドを受け付ける状態になり、入力されたコマンドに対応するパケットを送信します。

VirtualMachcine Version (vmv) パケットを送る例：
```
1> vmv
```

VirtualMachine ClassesBySigunature (vmcbs) パケットを送る例：
```
1> vmcbs Ljava/lang/String;
```

