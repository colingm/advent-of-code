#!/usr/bin/perl

use strict;
use warnings;
use experimental 'switch';

my $safe_count;
my $safe_count2;

while (<STDIN>) {
    chomp;
    my @line = split ' ', $_;
    my @safe_vals = ($line[0] - $line[1] > 0) ? 1..3 : -3..-1;

    my $safe = 1;
    for my $i (0..($#line - 1)) {
        my $diff = $line[$i] - $line[$i + 1];
        if (!($diff ~~ @safe_vals)) {
            $safe = 0;
            last;
        }
    }

    $safe_count += $safe;
    $safe_count2 += $safe;
    if (!$safe) {
        for my $i (0..$#line) { # Guess we are going to brute force this cause I'm lazy
            my @test = @line;
            splice(@test, $i, 1);
            my @safe_vals = ($test[0] - $test[1] > 0) ? 1..3 : -3..-1;
            my $safe2 = 1;
            for my $j (0..($#test - 1)) {
                my $diff = $test[$j] - $test[$j + 1];
                if (!($diff ~~ @safe_vals)) {
                    $safe2 = 0;
                    last;
                }
            }

            $safe_count2 += $safe2;
            if ($safe2) {
                last;
            }
        }
    }

}


print "PART 1: $safe_count\n";

print "Part 2: $safe_count2\n";