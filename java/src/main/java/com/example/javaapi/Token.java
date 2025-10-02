package com.example.javaapi;

import jakarta.persistence.*;
import java.time.LocalDate;
import java.time.LocalTime;

@Entity
@Table(name = "tokens")
public class Token {

    @Id
    private String id;

    private LocalDate date;
    private LocalTime time;
    private String batch;

    @Column(name = "created_at", insertable = false, updatable = false)
    private java.time.OffsetDateTime createdAt;

    // getters e setters
    public String getId() { return id; }
    public void setId(String id) { this.id = id; }

    public LocalDate getDate() { return date; }
    public void setDate(LocalDate date) { this.date = date; }

    public LocalTime getTime() { return time; }
    public void setTime(LocalTime time) { this.time = time; }

    public String getBatch() { return batch; }
    public void setBatch(String batch) { this.batch = batch; }

    public java.time.OffsetDateTime getCreatedAt() { return createdAt; }
}
