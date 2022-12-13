import java.io.File

fun main() {
    Day2.part1()
    Day2.part2()
}

object Day2 {
    fun part1() {
        val totalScore = linesToScores(
            ::outcomePointsOfRoundPart1, ::pointsForShapePart1
        ).fold(0) { total, round ->
            total + round.outcomePoints + round.shapePoints
        }
        println("Day 2 Part 1 Solution: $totalScore")
    }

    fun part2() {
        val totalScore = linesToScores(
            ::outcomePointsOfRoundPart2, ::pointsForShapePart2
        ).fold(0) { total, round ->
            total + round.outcomePoints + round.shapePoints
        }
        println("Day 2 Part 2 Solution: $totalScore")
    }

    private fun linesToScores(
        outcomePointsCalculator: (String, String) -> Int,
        shapePointsCalculator: (String, String) -> Int
    ): List<Score> {
        val scores: MutableList<Score> = mutableListOf()

        File("src/main/kotlin", "Day2.txt").useLines { lines ->
            lines.forEach { round ->
                val roundInput: List<String> = round.split(" ")
                if (roundInput.size != 2) {
                    throw IllegalArgumentException("A round should be 3 characters with the second being a space (e.g. A X)")
                }
                scores.add(
                    Score(
                        outcomePoints = outcomePointsCalculator(roundInput[0], roundInput[1]),
                        shapePoints = shapePointsCalculator(roundInput[0], roundInput[1])
                    )
                )
            }
        }

        return scores
    }

    private fun outcomePointsOfRoundPart1(other: String, selection: String): Int {
        return when (other) {
            "A" -> when (selection) {
                "X" -> 3
                "Y" -> 6
                "Z" -> 0
                else -> throw illegalSelectionInputException()
            }
            "B" -> when (selection) {
                "X" -> 0
                "Y" -> 3
                "Z" -> 6
                else -> throw illegalSelectionInputException()
            }
            "C" -> when (selection) {
                "X" -> 6
                "Y" -> 0
                "Z" -> 3
                else -> throw illegalSelectionInputException()
            }
            else -> throw IllegalArgumentException("Other must be in one \"A\", \"B\", or \"C\"")
        }
    }

    private fun outcomePointsOfRoundPart2(other: String, selection: String): Int {
        return when (selection) {
            "X" -> 0
            "Y" -> 3
            "Z" -> 6
            else -> throw illegalSelectionInputException()
        }
    }

    private fun pointsForShapePart1(other: String, selection: String): Int {
        return when (selection) {
            "X" -> 1
            "Y" -> 2
            "Z" -> 3
            else -> throw illegalSelectionInputException()
        }
    }

    private fun pointsForShapePart2(other: String, selection: String): Int {
        return when (other) {
            "A" -> when (selection) {
                "X" -> 3
                "Y" -> 1
                "Z" -> 2
                else -> throw illegalSelectionInputException()
            }
            "B" -> when (selection) {
                "X" -> 1
                "Y" -> 2
                "Z" -> 3
                else -> throw illegalSelectionInputException()
            }
            "C" -> when (selection) {
                "X" -> 2
                "Y" -> 3
                "Z" -> 1
                else -> throw illegalSelectionInputException()
            }
            else -> throw IllegalArgumentException("Other must be in one \"A\", \"B\", or \"C\"")
        }
    }

    private fun illegalSelectionInputException(): Exception {
        return IllegalArgumentException("Selection must be in one \"X\", \"Y\", or \"Z\"")
    }
}

data class Score(val outcomePoints: Int, val shapePoints: Int)
