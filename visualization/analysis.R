library(dplyr)
library(ggplot2)
library(cowplot)

input <- read.csv("summary_results.csv")
table(input$ruleSet)
pattern <- "^(.*)_(\\d+)_+(.*)$"
input_math_no_diff_int <- input %>%
    mutate(char = gsub("math_no_diff_int_", "", ruleSet)) %>%
    mutate(prefix = sub(pattern, "\\1",char)) %>%
    mutate(rule_number = sub(pattern, "\\2", char)) %>%
    mutate(suffix = gsub(pattern, "\\3", char))

no_div_no_pow_random_terms_huge <- input %>%
    # filter(testSet == "no_div_no_pow_random_terms_huge") %>%
    filter(grepl("no_div_no_pow_random_terms", testSet)) %>%
    filter(grepl("^math_no_diff_int_no_div_no_pow$|math_no_diff_int_no_div_no_pow_KBC_max_rules_100", ruleSet)) %>%
    dplyr::select(ruleSet, testSet, contains("X")) %>%
    reshape2::melt(id.vars = c("ruleSet", "testSet")) %>%
    mutate(variable = gsub("X", "", variable)) 
    # mutate(variable = as.numeric(variable))

p <- ggplot(no_div_no_pow_random_terms_huge, aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
    geom_line() +
    facet_wrap(. ~ testSet, scales = "free_y") +
    labs(
        title = "Comparison of Rule Sets on No Div No Pow Random Terms Huge Test Set",
        x = "Metric",
        y = "Value"
    ) +
    theme_minimal(base_size = 20) +
    guides(color = guide_legend(direction = "vertical")) +
    theme(legend.position = "bottom", )

plot_grid(p, p_log, ncol = 1, labels = c("A", "B"))
 
ggsave("no_div_no_pow_random_terms_huge_comparison.png", width = 16, height = 9)

no_div_no_pow_random_terms_large <- input %>%
    filter(testSet == "random_terms_large") %>%
    filter(!grepl("no_div_no_pow", ruleSet)) %>%
    filter(grepl("^math_no_diff_int$|150", ruleSet)) %>%
    dplyr::select(ruleSet, contains("X")) %>%
    reshape2::melt(id.vars = "ruleSet") %>%
    mutate(variable = gsub("X", "", variable))

p <- ggplot(no_div_no_pow_random_terms_large, aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
    geom_line() +
    labs(
        title = "Comparison of Rule Sets on No Div No Pow Random Terms Huge Test Set",
        x = "Metric",
        y = "Value"
    ) +
    theme_minimal(base_size = 25) +
    guides(color = guide_legend(direction = "vertical")) +    
    theme(legend.position = "bottom")
plot_grid(p,p, p, ncol = 3, nrow = 2)

#! for_plots.csv
for_plot <- read.csv("for_plots.csv", header = FALSE)
colnames(for_plot) <- c("ruleSet", "testSet", "40", "60", "80", "100", "150", "200", "1000", "2500")
p1 <- for_plot %>%
    filter(testSet == "no_div_no_pow_random_terms_huge") %>%
    reshape2::melt(id.vars = c("ruleSet", "testSet")) %>%
    mutate(variable = gsub("X", "", variable)) %>%
    mutate(variable = factor(variable, levels = c("40", "60", "80", "100", "150", "200", "1000", "2500"))) %>%
    ggplot(aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
    geom_line() +
    labs(
        title = NULL,
        x = "Rule Set Size",
        # y = "Average Symbol Difference"
        y = "Mean Symbol Reduction"
    ) +
    theme_linedraw(base_size = 18) +
    coord_cartesian(ylim = c(14, 27)) +
    guides(color = guide_legend(direction = "vertical")) +    
    theme(legend.position = "bottom")
p2 <- for_plot %>%
    filter(testSet == "random_terms_huge") %>%
    reshape2::melt(id.vars = c("ruleSet", "testSet")) %>%
    mutate(variable = gsub("X", "", variable)) %>%
    mutate(variable = factor(variable, levels = c("40", "60", "80", "100", "150", "200", "1000", "2500"))) %>%
    ggplot(aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
    geom_line() +
    labs(
        title = NULL,
        x = "Rule Set Size",
        y = "Mean Symbol Reduction"
    ) +
    theme_linedraw(base_size = 18) +
    coord_cartesian(ylim = c(13, 19)) +
    guides(color = guide_legend(direction = "vertical")) +    
    theme(legend.position = "bottom")
plot_grid(p1, p2, ncol = 1, labels = c("A", "B"))
ggsave("for_plot_no_div_no_pow_random_terms_huge.png", p1, width = 8, height = 6, bg = "white")
ggsave("for_plot_random_terms_huge.png", p2, width = 8, height = 6, bg = "white")

#! by_rule_postprocessing.csv
by_rule_postprocessing <- read.csv("by_rule_postprocessing.csv", header = FALSE)
colnames(by_rule_postprocessing) <- c("ruleSet", "testSet", "0.0001", "0.0005", "0.001", "0.005", "0.01", "0.05", "0.1", "0.5", "1.0")
p1 <- by_rule_postprocessing %>%
    filter(testSet == "no_div_no_pow_random_terms_huge") %>%
    reshape2::melt(id.vars = c("ruleSet", "testSet")) %>%
    mutate(variable = gsub("X", "", variable)) %>%
    mutate(variable = factor(variable, levels = c("0.0001", "0.0005", "0.001", "0.005", "0.01", "0.05", "0.1", "0.5", "1.0"))) %>%
    ggplot(aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
    geom_line() +
    labs(
        title = NULL,
        x = "Time Limit",
        y = "Mean Symbol Reduction"
    ) +
    theme_linedraw(base_size = 18) +
    guides(color = guide_legend(direction = "vertical")) +    
    theme(legend.position = "bottom")
p2 <- by_rule_postprocessing %>%
    filter(testSet == "random_terms_huge") %>%
    reshape2::melt(id.vars = c("ruleSet", "testSet")) %>%
    mutate(variable = gsub("X", "", variable)) %>%
    mutate(variable = factor(variable, levels = c("0.0001", "0.0005", "0.001", "0.005", "0.01", "0.05", "0.1", "0.5", "1.0"))) %>%
    ggplot(aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
    geom_line() +
    labs(
        title = NULL,
        x = "Time Limit",
        y = "Mean Symbol Reduction"
    ) +
    theme_linedraw(base_size = 18) +
    guides(color = guide_legend(direction = "vertical")) +    
    theme(legend.position = "bottom")
ggsave("by_rule_postprocessing_no_div_no_pow_random_terms_huge.png", p1, width = 8, height = 6, bg = "white")
ggsave("by_rule_postprocessing_random_terms_huge.png", p2, width = 8, height = 6, bg = "white")

#! by_rule_number.csv
by_rule_number <- read.csv("by_rule_number.csv", header = FALSE)
colnames(by_rule_number) <- c("ruleSet", "testSet", "0.0001", "0.0005", "0.001", "0.005", "0.01", "0.05", "0.1", "0.5", "1.0")
p1 <- by_rule_number %>%
    filter(grepl("extended", ruleSet)) %>%
    reshape2::melt(id.vars = c("ruleSet", "testSet")) %>%
    mutate(variable = gsub("X", "", variable)) %>%
        mutate(variable = factor(variable, levels = c("0.0001", "0.0005", "0.001", "0.005", "0.01", "0.05", "0.1", "0.5", "1.0"))) %>%
        ggplot(aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
        geom_line() +
        labs(
            title = NULL,
            x = "Time Limit",
            y = "Mean Symbol Reduction"
        ) +
        theme_linedraw(base_size = 18) +
        guides(color = guide_legend(direction = "vertical")) +
        theme(legend.position = "bottom")
p2 <- by_rule_number %>%
    filter(!grepl("extended", ruleSet)) %>%
    reshape2::melt(id.vars = c("ruleSet", "testSet")) %>%
    mutate(variable = gsub("X", "", variable)) %>%
        mutate(variable = factor(variable, levels = c("0.0001", "0.0005", "0.001", "0.005", "0.01", "0.05", "0.1", "0.5", "1.0"))) %>%
        ggplot(aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
        geom_line() +
        labs(
            title = NULL,
            x = "Time Limit",
            y = "Mean Symbol Reduction"
        ) +
        theme_linedraw(base_size = 18) +
        guides(color = guide_legend(direction = "vertical")) +
        theme(legend.position = "bottom")
ggsave("by_rule_number_extended.png", p1, width = 8, height = 6, bg = "white")
ggsave("by_rule_number_standard.png", p2, width = 8, height = 6, bg = "white")

#! by_rule_set.csv
by_rule_set <- read.csv("by_rule_set.csv", header = FALSE)
colnames(by_rule_set) <- c("ruleSet", "testSet", "0.0001", "0.0005", "0.001", "0.005", "0.01", "0.05", "0.1", "0.5", "1.0")
p1 <- by_rule_set %>%
    filter(grepl("no_div_no_pow_random_terms_small", testSet)) %>%
    reshape2::melt(id.vars = c("ruleSet", "testSet")) %>%
    mutate(variable = gsub("X", "", variable)) %>%
        mutate(variable = factor(variable, levels = c("0.0001", "0.0005", "0.001", "0.005", "0.01", "0.05", "0.1", "0.5", "1.0"))) %>%
        ggplot(aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
        geom_line() +
        labs(
            title = NULL,
            x = "Time Limit",
            y = "Mean Symbol Reduction"
        ) +
        theme_linedraw(base_size = 18) +
        guides(color = guide_legend(direction = "vertical")) +
        theme(legend.position = "bottom")

p2 <- by_rule_set %>%
    filter(grepl("no_div_no_pow_random_terms_large", testSet)) %>%
    reshape2::melt(id.vars = c("ruleSet", "testSet")) %>%
    mutate(variable = gsub("X", "", variable)) %>%
        mutate(variable = factor(variable, levels = c("0.0001", "0.0005", "0.001", "0.005", "0.01", "0.05", "0.1", "0.5", "1.0"))) %>%
        ggplot(aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
        geom_line() +
        labs(
            title = NULL,
            x = "Time Limit",
            y = "Mean Symbol Reduction"
        ) +
        theme_linedraw(base_size = 18) +
        guides(color = guide_legend(direction = "vertical")) +
        theme(legend.position = "bottom")

p3 <- by_rule_set %>%
    filter(grepl("no_div_no_pow_random_terms_huge", testSet)) %>%
    reshape2::melt(id.vars = c("ruleSet", "testSet")) %>%
    mutate(variable = gsub("X", "", variable)) %>%
        mutate(variable = factor(variable, levels = c("0.0001", "0.0005", "0.001", "0.005", "0.01", "0.05", "0.1", "0.5", "1.0"))) %>%
        ggplot(aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
        geom_line() +
        labs(
            title = NULL,
            x = "Time Limit",
            y = "Mean Symbol Reduction"
        ) +
        theme_linedraw(base_size = 18) +
        guides(color = guide_legend(direction = "vertical")) +
        theme(legend.position = "bottom")
ggsave("by_rule_set_no_div_no_pow_random_terms_small.png", p1, width = 8, height = 6, bg = "white")
ggsave("by_rule_set_no_div_no_pow_random_terms_large.png", p2, width = 8, height = 6, bg = "white")
ggsave("by_rule_set_no_div_no_pow_random_terms_huge.png", p3, width = 8, height = 6, bg = "white")

p1 <- by_rule_set %>%
    filter(testSet == "random_terms_small") %>%
    reshape2::melt(id.vars = c("ruleSet", "testSet")) %>%
    mutate(variable = gsub("X", "", variable)) %>%
    mutate(variable = factor(variable, levels = c("0.0001", "0.0005", "0.001", "0.005", "0.01", "0.05", "0.1", "0.5", "1.0"))) %>%
    ggplot(aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
    geom_line() +
    labs(
        title = NULL,
        x = "Time Limit",
        y = "Mean Symbol Reduction"
    ) +
    theme_linedraw(base_size = 18) +
    guides(color = guide_legend(direction = "vertical")) +
    theme(legend.position = "bottom")
p2 <- by_rule_set %>%
    filter(testSet == "random_terms_large") %>%
    reshape2::melt(id.vars = c("ruleSet", "testSet")) %>%
    mutate(variable = gsub("X", "", variable)) %>%
        mutate(variable = factor(variable, levels = c("0.0001", "0.0005", "0.001", "0.005", "0.01", "0.05", "0.1", "0.5", "1.0"))) %>%
    ggplot(aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
    geom_line() +
    labs(
        title = NULL,
        x = "Time Limit",
        y = "Mean Symbol Reduction"
    ) +
    theme_linedraw(base_size = 18) +
    guides(color = guide_legend(direction = "vertical")) +
    theme(legend.position = "bottom")
p3 <- by_rule_set %>%
    filter(testSet == "random_terms_huge") %>%
    reshape2::melt(id.vars = c("ruleSet", "testSet")) %>%
    mutate(variable = gsub("X", "", variable)) %>%
        mutate(variable = factor(variable, levels = c("0.0001", "0.0005", "0.001", "0.005", "0.01", "0.05", "0.1", "0.5", "1.0"))) %>%
    ggplot(aes(x = variable, y = value, color = ruleSet, group = ruleSet)) +
    geom_line() +
    labs(
        title = NULL,
        x = "Time Limit",
        y = "Mean Symbol Reduction"
    ) +
    theme_linedraw(base_size = 18) +
    guides(color = guide_legend(direction = "vertical")) +
    theme(legend.position = "bottom")
ggsave("by_rule_set_random_terms_small.png", p1, width = 8, height = 6, bg = "white")
ggsave("by_rule_set_random_terms_large.png", p2, width = 8, height = 6, bg = "white")
ggsave("by_rule_set_random_terms_huge.png", p3, width = 8, height = 6, bg = "white")
