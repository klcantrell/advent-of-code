fun main() {
    val testInput = readInput("Day01_test")
    check(part1(testInput) == 3)

    val input = readInput("Day01")
    println(part1(input))
    println(part2(input))
}

fun part1(input: List<String>): Int {
    return input
        .zip(input.indices)
        .drop(1)
        .map { (currentValue, index) ->
            val previousValue = input[index - 1]
            if (currentValue.toInt() > previousValue.toInt()) {
                Direction.INC
            } else if (currentValue.toInt() < previousValue.toInt()) {
                Direction.DEC
            } else {
                Direction.NO_CHANGE
            }
        }
        .fold(0) { result, current ->
            when (current) {
                Direction.INC -> result + 1
                else -> result
            }
        }
}

fun part2(input: List<String>): Int {
    return input.size
}

enum class Direction {
    INC, DEC, NO_CHANGE
}
