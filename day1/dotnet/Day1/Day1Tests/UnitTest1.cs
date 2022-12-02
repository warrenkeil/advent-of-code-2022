using System.Collections.Generic;
using System.Net.Mime;
using Day1;
using NUnit.Framework;

namespace Day1Tests;

public class Tests
{
  private IEnumerable<string?> elfCalorieData; 
  
  [SetUp]
  public void Setup()
  {
    DataProcessor dataProcessor = new DataProcessor();

    elfCalorieData = dataProcessor.ProcessDataFile("testData.txt");

  }

  [Test]
  public void CountElfCalories_WhenGivenTestArray_ShouldReturnCorrectCalories()
  {
    var highestCalories = CalorieCounter.CountElfCalories(elfCalorieData);
    Assert.AreEqual(24000, highestCalories);
  }
}