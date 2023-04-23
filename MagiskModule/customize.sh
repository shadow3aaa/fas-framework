SKIPUNZIP=0
echo "----------------------------------------------------"

echo -e "\nPlease wait…"
echo "请等待…"

# permission
chmod a+x $MODPATH/fas-framework
# start on install
killall fas-framework > /dev/null 2>&1
nohup $MODPATH/fas-framework > $MODPATH/Errors 2>&1 &
echo "----------------------------------------------------"