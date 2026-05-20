# visualize.gp
set terminal pngcairo size 800,400
set output "fwht_matrices.png"

set multiplot layout 1,2 title "Walsh-Hadamard Transform Matrices (64x64)"

# Remove axes, tics, and colorbox for clean images
unset xtics
unset ytics
unset colorbox

# Flip the y-axis so row 0 is at the top
set yrange [*:*] reverse

# Define a simple black and white palette (-1 = black, 1 = white)
set palette defined (-1 "black", 1 "white")

set title "Natural (Sylvester) Ordering"
plot 'natural.dat' matrix with image

set title "Sequency (Walsh) Ordering"
plot 'sequency.dat' matrix with image

unset multiplot
