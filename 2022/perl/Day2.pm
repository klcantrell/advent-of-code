package Day2;

use strict;
use warnings;

use constant INVALID_INPUT => "Other must be in one \"A\", \"B\", or \"C\". Selection must be in one \"X\", \"Y\", or \"Z\"";

sub part1 {
    my @scores = _lines_to_scores(
        \&_outcome_points_of_round_part1,
        \&_points_for_shape_part1
    );
    my $total_score = 0;
    foreach my $score (@scores) {
        $total_score += $score->{outcome_points} + $score->{shape_points};
    }

    print("Day 2 Part 1 solution: $total_score\n");
}

sub part2 {
    my @scores = _lines_to_scores(
        \&_outcome_points_of_round_part2,
        \&_points_for_shape_part2
    );
    my $total_score = 0;
    foreach my $score (@scores) {
        $total_score += $score->{outcome_points} + $score->{shape_points};
    }

    print("Day 2 Part 2 solution: $total_score\n")
}

sub _outcome_points_of_round_part1 {
    my $other = shift;
    my $selection = shift;

    if ($other eq 'A' && $selection eq 'X') {
        return 3;
    }
    elsif ($other eq 'A' && $selection eq 'Y') {
        return 6;
    }
    elsif ($other eq 'A' && $selection eq 'Z') {
        return 0;
    }
    elsif ($other eq 'B' && $selection eq 'X') {
        return 0;
    }
    elsif ($other eq 'B' && $selection eq 'Y') {
        return 3;
    }
    elsif ($other eq 'B' && $selection eq 'Z') {
        return 6;
    }
    elsif ($other eq 'C' && $selection eq 'X') {
        return 6;
    }
    elsif ($other eq 'C' && $selection eq 'Y') {
        return 0;
    }
    elsif ($other eq 'C' && $selection eq 'Z') {
        return 3;
    }
    else {
        die INVALID_INPUT;
    }
}

sub _points_for_shape_part1 {
    my $other = shift;
    my $selection = shift;

    if ($selection eq 'X') {
        return 1;
    }
    elsif ($selection eq 'Y') {
        return 2;
    }
    elsif ($selection eq 'Z') {
        return 3;
    }
    else {
        die INVALID_INPUT;
    }
}

sub _outcome_points_of_round_part2 {
    my $other = shift;
    my $selection = shift;

    if ($selection eq 'X') {
        return 0;
    }
    elsif ($selection eq 'Y') {
        return 3;
    }
    elsif ($selection eq 'Z') {
        return 6;
    }
    else {
        die INVALID_INPUT;
    }
}

sub _points_for_shape_part2 {
    my $other = shift;
    my $selection = shift;

    if ($other eq 'A' && $selection eq 'X') {
        return 3;
    }
    elsif ($other eq 'A' && $selection eq 'Y') {
        return 1;
    }
    elsif ($other eq 'A' && $selection eq 'Z') {
        return 2;
    }
    elsif ($other eq 'B' && $selection eq 'X') {
        return 1;
    }
    elsif ($other eq 'B' && $selection eq 'Y') {
        return 2;
    }
    elsif ($other eq 'B' && $selection eq 'Z') {
        return 3;
    }
    elsif ($other eq 'C' && $selection eq 'X') {
        return 2;
    }
    elsif ($other eq 'C' && $selection eq 'Y') {
        return 3;
    }
    elsif ($other eq 'C' && $selection eq 'Z') {
        return 1;
    }
    else {
        die INVALID_INPUT;
    }
}

sub _lines_to_scores {
    my $outcome_points_calculator = shift;
    my $shape_points_calculator = shift;

    my @scores = ();

    open(my $fh, '<', 'Day2.txt') or die "Failed to read input file: $!";

    while (my $line = <$fh>) {
        chomp $line;
        my @round_input = split(" ", $line);
        if (@round_input != 2) {
            die "A round should be 3 characters with the second being a space (e.g. A X)";
        }

        push(@scores,
            {
                outcome_points => $outcome_points_calculator->($round_input[0], $round_input[1]),
                shape_points   => $shape_points_calculator->($round_input[0], $round_input[1]),
            }
        );

    }

    return @scores;
}

1;
