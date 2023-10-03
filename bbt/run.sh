#!/usr/bin/env bash

echo " "
echo "────────────────────────────────────"
echo " Tested version: $(biff -V)"
echo "────────────────────────────────────"

RUN_SCRIPT=run.sh
WORK_DIR=$(pwd)
ROOT_DIR=./tests

PASSED=0
FAILED=0

run_test () {
  cd "$1" || exit 1
  if [ -f "actual" ]; then
    rm actual
  fi
  ./"$2" > actual
  diff expected actual >> /dev/null 2>&1

  case $? in
    0)
      PASSED=$((PASSED+1))
      rm actual
      echo "$1/$2....OK"
      ;;

    *)
      FAILED=$((FAILED+1))
      echo "$1/$2....FAILED"
      echo ""
      echo -e "\e[31mEXPECTED\e[0m                                                                                                \e[32mACTUAL\e[0m"
      echo -e "\e[31m─────────────────────────────────────────────────────────────────────────────────────────────────\e[0m       \e[32m─────────────────────────────────────────────────────────────────────────────────────────────────\e[0m"
      diff expected actual -y --color=always -W 200
      echo ""
      echo ""
      echo "Full report:"
      diff actual expected --color=always
      rm actual
      ;;
  esac

  cd "$WORK_DIR" || exit 1
}

for dir in $(find $ROOT_DIR -type d | sort); do
  if [ -f "$dir/$RUN_SCRIPT" ]; then
    run_test "$dir" "$RUN_SCRIPT"
  fi
done

echo "────────────────────────────────────"
echo " $PASSED passed, $FAILED failed"
echo "────────────────────────────────────"
echo " "

exit 0