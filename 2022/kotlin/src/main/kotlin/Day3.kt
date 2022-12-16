import java.io.File

fun main() {
    Day3.part1()
    Day3.part2()
}

object Day3 {
    fun part1() {
        val commonItems = linesToRucksacks()
            .map(::findCommonItemType)
            .map(::priorityOfItemType)
        println("Day 3 Part 1 solution: ${commonItems.sum()}")
    }

    fun part2() {
        val badgesForEachGroup = linesToRucksacks()
            .map { it.compartmentOne + it.compartmentTwo }
            .chunked(3)
            .map(::findCommonItemType)
            .map(::priorityOfItemType)
        println("Day 3 Part 2 solution: ${badgesForEachGroup.sum()}")
    }

    private fun linesToRucksacks(): List<Rucksack> {
        val rucksacks: MutableList<Rucksack> = mutableListOf()

        File("src/main/kotlin", "Day3.txt").useLines { lines ->
            lines.forEach { line ->
                rucksacks.add(
                    Rucksack(
                        line.substring(0, line.length / 2),
                        line.substring(line.length / 2)
                    )
                )
            }
        }

        return rucksacks
    }

    private fun findCommonItemType(rucksack: Rucksack): String {
        return (rucksack.compartmentOne.toSet() intersect rucksack.compartmentTwo.toSet())
            .first()
            .toString()
    }

    private fun findCommonItemType(emptiedRucksacks: List<String>): String {
        if (emptiedRucksacks.size > 2) {
            val result = mutableListOf(
                (emptiedRucksacks[0].toSet() intersect emptiedRucksacks[1].toSet())
                    .joinToString("")
            )
            result.addAll(emptiedRucksacks.drop(2))
            return findCommonItemType(result)
        }

        return (emptiedRucksacks[0].toSet() intersect emptiedRucksacks[1].toSet()).first().toString()
    }

    private fun priorityOfItemType(item: String): Int {
        return itemTypePriority[item]!!
    }
}

data class Rucksack(val compartmentOne: String, val compartmentTwo: String)

val itemTypePriority = mapOf(
    "a" to 1,
    "b" to 2,
    "c" to 3,
    "d" to 4,
    "e" to 5,
    "f" to 6,
    "g" to 7,
    "h" to 8,
    "i" to 9,
    "j" to 10,
    "k" to 11,
    "l" to 12,
    "m" to 13,
    "n" to 14,
    "o" to 15,
    "p" to 16,
    "q" to 17,
    "r" to 18,
    "s" to 19,
    "t" to 20,
    "u" to 21,
    "v" to 22,
    "w" to 23,
    "x" to 24,
    "y" to 25,
    "z" to 26,
    "A" to 27,
    "B" to 28,
    "C" to 29,
    "D" to 30,
    "E" to 31,
    "F" to 32,
    "G" to 33,
    "H" to 34,
    "I" to 35,
    "J" to 36,
    "K" to 37,
    "L" to 38,
    "M" to 39,
    "N" to 40,
    "O" to 41,
    "P" to 42,
    "Q" to 43,
    "R" to 44,
    "S" to 45,
    "T" to 46,
    "U" to 47,
    "V" to 48,
    "W" to 49,
    "X" to 50,
    "Y" to 51,
    "Z" to 52,
)
