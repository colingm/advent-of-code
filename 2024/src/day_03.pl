#!/usr/bin/perl

use strict;
use warnings;

no warnings 'uninitialized';

my $input;
my $sum;
my $sum2;

while (<STDIN>) {
    chomp;
    $input .= $_; # I want the whole file without any line breaks to make part 2 work
}
while ($input =~ /mul\((\d+),(\d+)\)/g) {
    $sum += $1 * $2;
}

while ($input =~ /(?:(?<=don't\(\)).*?(?=do\(\)|$))|(?:mul\((\d+),(\d+)\))/g) {
    $sum2 += $1 * $2;
}



print "PART 1: $sum\n";

print "Part 2: $sum2\n";