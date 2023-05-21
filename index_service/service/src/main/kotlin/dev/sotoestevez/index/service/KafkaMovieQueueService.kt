package dev.sotoestevez.index.service

import dev.sotoestevez.index.dto.MovieDoc
import org.apache.kafka.clients.admin.NewTopic
import org.springframework.kafka.core.KafkaTemplate
import org.springframework.stereotype.Service

@Service
class KafkaMovieQueueService(
    private val kafkaTemplate: KafkaTemplate<String, String>,
    private val rawDocTopic: NewTopic
): QueueService<MovieDoc> {

    override fun enqueue(doc: MovieDoc): Boolean {
        // todo map doc
        kafkaTemplate.send(rawDocTopic.name(), doc.title)
        // todo log errors
        return true
    }

    override fun enqueue(docs: List<MovieDoc>): Boolean {
        return docs.map(this::enqueue).all{ it }
    }


}