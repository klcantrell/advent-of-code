import java.io.File

fun main()  {
    part1()
    part2()
}

fun part1() {
    println("Day 1 Part 1 Solution: ${caloriesByElf().max()}")
}

fun part2() {
    val caloriesByElfDescending = caloriesByElf().sortedDescending()
    println("Day 1 Part 2 Solution: ${caloriesByElfDescending.take(3).sum()}")
}

fun caloriesByElf(): List<Int> {
    val parsedData: MutableList<List<Int>> = mutableListOf()
    var group: MutableList<Int> = mutableListOf()

    File("src/main/kotlin", "Day1.txt").useLines { lines ->
        lines.forEach { line ->
            if (line.isEmpty()) {
                parsedData.add(group)
                group = mutableListOf()
            } else {
                line.toIntOrNull()?.let {
                    group.add(it)
                }
            }
        }

        if (group.isNotEmpty()) {
            parsedData.add(group)
        }
    }

    return parsedData.map { it.sum() }
}