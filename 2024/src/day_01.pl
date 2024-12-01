#!/usr/bin/perl

use strict;
use warnings;

no warnings 'uninitialized';

my @left;
my @right;

while (<STDIN>) {
    chomp;
    my @line = split ' ', $_;

    push @left, $line[0];
    push @right, $line[1];
}

my @sorted_left = sort {$a <=> $b} @left;
my @sorted_right = sort {$a <=> $b} @right;

my $sum;

for my $i (0 .. $#left) {
    $sum += abs $sorted_left[$i] - $sorted_right[$i];
}

print "PART 1: $sum\n";

my %frequency;

foreach (@sorted_right) {
    $frequency{$_}++;
}

# print "$_ => $frequency{$_}\n" for (keys %frequency);

my $similarity;
foreach (@sorted_left) {
    $similarity += ($_ * $frequency{$_});
}

print "Part 2: $similarity\n";