ACCEL_FACTOR=${1:-0.007}
CAP=${2:-2}
STEP=1

cat <<EOL
Section "InputClass"
    Identifier "My Mouse"
    Driver "libinput"
    MatchIsPointer "yes"

    Option "AccelProfile" "custom"
    Option "AccelStepMotion" "$STEP"
    Option "AccelPointsMotion" "$(libinput-points $ACCEL_FACTOR $CAP $STEP)"
EndSection
EOL
