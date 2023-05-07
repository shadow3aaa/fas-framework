#!/system/bin/sh
MODDIR=${0%/*}

wait_until_login() {
    while [ "$(getprop sys.boot_completed)" != "1" ]; do
        sleep 1
    done
    while [ ! -d "/sdcard/Android" ]; do
        sleep 1
    done
}
wait_until_login

chmod a+x $MODDIR/fas-framework
nohup $MODDIR/fas-framework > $MODDIR/Errors 2>&1 &