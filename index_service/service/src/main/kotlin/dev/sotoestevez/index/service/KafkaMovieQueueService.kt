package dev.sotoestevez.index.service

import dev.sotoestevez.index.dto.MovieDoc
import org.springframework.stereotype.Service

@Service
class KafkaMovieQueueService: QueueService<MovieDoc> {
    override fun enqueue(doc: MovieDoc): Boolean {
        println("Enqueued: ${doc.title}")
        return true
    }

    override fun enqueue(docs: List<MovieDoc>): Boolean {
        return docs.map(this::enqueue).all{ it }
    }


}