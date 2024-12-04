#!/usr/bin/perl

use strict;
use warnings;

my @input;
my $sum;
my $sum2;

while (<STDIN>) {
    chomp;
    my @row = split(//, $_);
    push(@input, \@row);
}

my $rows = $#input;
my $cols = $#{$input[0]};

my @dirs = ([-1,-1],[-1,0],[-1,1],[0,-1],[0, 1],[1,-1],[1,0],[1,1]);

for my $i (0..$rows) {
    for my $j (0..$cols) {
        foreach (@dirs) {
            my $cur = $input[$i]->[$j];
            for my $x (1..3) {
                my $ni = $i + ($x * $_->[0]);
                my $nj = $j + ($x * $_->[1]);
                if ($ni < 0 || $ni > $rows || $nj < 0 || $nj > $cols) { last; }
                $cur .= $input[$ni]->[$nj];
            }

            $sum += $cur eq "XMAS"
        }
    }
}

for my $i (1..($rows-1)) {
    for my $j (1..($cols-1)) {
        if ($input[$i]->[$j] ne "A") { next; }
        my $ttb = $input[$i - 1]->[$j - 1] . $input[$i]->[$j] . $input[$i + 1]->[$j + 1];
        my $btt = $input[$i + 1]->[$j - 1] . $input[$i]->[$j] . $input[$i - 1]->[$j + 1];
        $sum2 += ($ttb eq "MAS" || $ttb eq "SAM") && ($btt eq "MAS" || $btt eq "SAM");
    }
}

print "PART 1: $sum\n";

print "Part 2: $sum2\n";