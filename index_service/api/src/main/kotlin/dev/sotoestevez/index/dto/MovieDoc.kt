package dev.sotoestevez.index.dto

data class MovieDocs(
    val docs: List<MovieDoc>
)

data class MovieDoc(
    val id: Int,
    val title: String,
    val releaseDate: String?,    // todo map to date
    val runtime: Int,
    val overview: String,
    val genres: List<MovieGenre>,
    val posterPath: String?,
    val popularity: Int,
    val voteAverage: Float,
    val voteCount: Int,
    val originalTitle: String?,
    val adult: Boolean,
    val budget: Int,
    val revenue: Int,
    val homepage: String,
    val imdbId: String?
)

data class MovieGenre(
    val id: Int,
    val name: String
)