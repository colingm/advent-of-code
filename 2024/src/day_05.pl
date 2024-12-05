#!/usr/bin/perl

use strict;
use warnings;
use feature 'say';
use Data::Dumper;

my $part1;
my $part2;

my @rules;
my @updates;
my @update_orders;

while (<STDIN>) {
    chomp;
    if ($_ =~ /(\d+)\|(\d+)/) {
        my @rule = ($1, $2);
        push (@rules, \@rule);
    } else {
        my @update;
        while ($_ =~ /,?(\d+)/g) {
            push(@update, $1);
        }


        if (@update) {
            my %update_order;
            for my $i (0..$#update) {
                $update_order{$update[$i]} = $i;
            }
            push(@updates, \@update);
            push(@update_orders, \%update_order);
        }
    }
}

for my $i (0..$#updates) {
    my $update = $updates[$i];
    my $order = $update_orders[$i];
    my $valid = 1;
    foreach (@rules) {
        my $first = $order->{$_->[0]};
        my $second = $order->{$_->[1]};
        if (defined $first && defined $second && $first > $second) {
            $valid = 0;
            last;
        }
    }

    if ($valid) {
        my $val = $update->[int(@$update/2)];
        $part1 += $val;
    }

    my $j = 0;
    my $invalid = 0;
    while ($j <= $#rules) {
        my $first = $order->{$rules[$j]->[0]};
        my $second = $order->{$rules[$j]->[1]};
        if (defined $first && defined $second && $first > $second) {
            $invalid = 1;
            splice(@{$update}, $second, 0, splice(@{$update}, $first, 1));
            for my $k (0..$#{$update}) {
                $order->{$update->[$k]} = $k;
            }
            $j = 0;
        } else {
            $j++;
        }
    }

    if ($invalid) {
        my $val = $update->[int(@$update/2)];
        $part2 += $val;
    }
}

say "PART 1: $part1";
say "PART 2: $part2";