
using System;
using System.Collections.Generic;
using System.Linq;
using Day1;

var dataProcessor = new DataProcessor();

var elfData = dataProcessor.ProcessDataFile("data.txt");
Console.WriteLine($"The max number of calories is {CalorieCounter.CountElfCalories(elfData)}");
Console.WriteLine($"The sum of calories for the top three elves is {CalorieCounter.CountElfCalories(elfData, true)}");
public static class CalorieCounter
{
  public static int CountElfCalories(IEnumerable<string?> elfFoodString, bool isTopThree = false)
  {
    var elfCalorieList = new List<int>();
    var sum = 0;
    foreach (var value in elfFoodString)
    {
      if (value != "")
      {
        var intValue = Int32.Parse(value!);
        sum += intValue;
      }
      if (value == "")
      {
        elfCalorieList.Add(sum);
        sum = 0;
      }
    }

    if (isTopThree)
    {
      return elfCalorieList.OrderByDescending(x => x).Take(3).Sum();
    }
    return elfCalorieList.Max();
  }
}