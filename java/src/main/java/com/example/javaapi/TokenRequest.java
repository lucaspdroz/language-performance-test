package com.example.javaapi;

import java.time.LocalDate;
import java.time.LocalTime;

public class TokenRequest {
    private LocalDate date;
    private LocalTime time;
    private String batch;

    // getters e setters
    public LocalDate getDate() { return date; }
    public void setDate(LocalDate date) { this.date = date; }

    public LocalTime getTime() { return time; }
    public void setTime(LocalTime time) { this.time = time; }

    public String getBatch() { return batch; }
    public void setBatch(String batch) { this.batch = batch; }
}
