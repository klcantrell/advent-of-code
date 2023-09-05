package Day1;

use strict;
use warnings;

sub part1 {
    my @calories_sorted = sort { $b <=> $a } _calories_by_elf();
    print("Day 1 Part 1 solution: $calories_sorted[0]\n");
}

sub part2 {
    my @calories_sorted = sort { $b <=> $a } _calories_by_elf();
    my $top_three = 0;
    foreach my $n (@calories_sorted[0..2]) {
        $top_three += $n;
    }
    print("Day 1 Part 2 solution: $top_three\n")
}

sub _calories_by_elf {
    my @parsed_data = ();
    my @group = ();

    open(my $fh, '<', 'Day1.txt') or die "Failed to read input file: $!";

    while (my $line = <$fh>) {
        chomp $line;
        if ($line eq '') {
            push(@parsed_data, [@group]);
            @group = ();
        } else {
            push(@group, $line);
        }
    }

    if (@group) {
        push(@parsed_data, [@group]);
    }

    map {
        my $sum = 0;
        foreach my $n (@$_) {
            $sum += $n;
        }
        $sum
    } @parsed_data
}

1;
