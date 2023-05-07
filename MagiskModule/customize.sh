SKIPUNZIP=0
ui_print "----------------------------------------------------"

ui_print "请等待…"
ui_print "Please wait…"

# permission
chmod a+x $MODPATH/fas-framework

# start on install
killall fas-framework > /dev/null 2>&1
nohup $MODPATH/fas-framework > $MODPATH/Errors 2>&1 &

ui_print "----------------------------------------------------"